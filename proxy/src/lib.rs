#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling proxying to another Minecraft server.

use std::io::{self, Cursor};

use log::{error, info, warn};
use mc_coms::{
    client_state::ClientState,
    codec::{json_string::JsonString, prefixed_array::PrefixedArray, var_int::VarInt},
    key_store::{EncryptionError, KeyStore},
    messages::{
        McPacketError, McPacketRead,
        clientbound::{
            configuration::{
                clientbound_keep_alive::ClientboundKeepAlive,
                clientbound_known_packs::ClientboundKnownPacks,
                clientbound_plugin_message::ClientboundPluginMessage, feature_flags::FeatureFlags,
                registry_data::RegistryData, update_tags::UpdateTags,
            },
            login::encryption_request::EncryptionRequest,
            play::{
                change_difficulty::ChangeDifficulty, login::Login,
                player_abilities::PlayerAbilities, set_held_item::SetHeldItem,
                update_recipes::UpdateRecipes,
            },
            status::{pong_response::PongResponse, status_response::ServerStatus},
        },
        serverbound::{
            configuration::{
                client_information::ClientInformation,
                serverbound_known_packs::ServerboundKnownPacks,
                serverbound_plugin_message::ServerboundPluginMessage,
            },
            handshaking::handshake::Handshake,
            login::{encryption_response::EncryptionResponse, login_start::LoginStart},
            status::ping_request::PingRequest,
        },
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    packet_writer::{NetworkWriter, PacketWriteError},
    ser::{NetworkWriteExt, ReadingError, WritingError},
};
use owo_colors::OwoColorize;
use rand::Rng;
use serde::Deserialize;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

/// An error that occurrs during proxying.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum ProxyError {
    #[error("Failed setting up proxy connection to remove server, inner error: `{0}`")]
    FailedStartingServerComs(io::Error),
    #[error("Failed to read packet, error: `{0}`")]
    PacketReadError(#[from] PacketReadError),
    #[error("Failed to write packet, error: `{0}`")]
    WriteError(#[from] WritingError),
    #[error("IO Error, error: `{0}`")]
    IoError(#[from] io::Error),
    #[error("Invalid packet received")]
    InvalidPacket,
    #[error("Failed to deserialize packet")]
    PacketDeserializationError(#[from] ReadingError),
    #[error("Failed to write packet")]
    PacketWriteError(#[from] PacketWriteError),
    #[error("An encryption error occurred, err: {0}")]
    EncryptionError(#[from] EncryptionError),
    #[error("UUID parse error")]
    UuidError(#[from] uuid::Error),
    #[error("MC Packet Error: {0}")]
    McPacketError(#[from] McPacketError),
}

/// Handling connection for the proxy.
pub struct ProxyHandler<'key> {
    client_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    client_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    server_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    server_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    key_store: &'key KeyStore,
    state: ClientState,
    handling_packet: bool,
}

impl<'key> ProxyHandler<'key> {
    /// Creates a [ProxyHandler] from the provided [stream].
    pub async fn new(
        stream: TcpStream,
        target: &str,
        key_store: &'key KeyStore,
    ) -> Result<Self, ProxyError> {
        let (client_reader, client_writer) = stream.into_split();
        let client_reader = NetworkReader::new(BufReader::new(client_reader));
        let client_writer = NetworkWriter::new(BufWriter::new(client_writer));

        let out_stream = TcpStream::connect(target)
            .await
            .map_err(ProxyError::FailedStartingServerComs)?;
        info!("Connection setup to {target}");

        let (server_reader, server_writer) = out_stream.into_split();
        let server_reader = NetworkReader::new(BufReader::new(server_reader));
        let server_writer = NetworkWriter::new(BufWriter::new(server_writer));

        Ok(ProxyHandler {
            client_reader,
            client_writer,
            server_reader,
            server_writer,
            key_store,
            state: ClientState::Handshaking,
            handling_packet: false,
        })
    }

    /// Start the [ProxyHandler] and handle connections.
    pub async fn run(&mut self) -> Result<(), ProxyError> {
        loop {
            println!("\n\n");
            tokio::select! {
                to_server = self.client_reader.get_packet() => {
                    let packet = match to_server {
                        Ok(p) => p,
                        Err(PacketReadError::ConnectionClosed) => {
                            warn!("Connection to client was closed");
                            return Ok(())
                        }
                        Err(e) => return Err(e.into())
                    };

                    self.log_server_bound(packet.id, format!("Packet to server {packet:02x?}").green().to_string().as_str());
                    self.handling_packet = true;

                    match self.parse_and_log_server_bound_packet(packet.clone()).await {
                        Ok(true) => { /* The server has been dealt with */ }
                        Ok(false) => {
                            self.log_server_bound(packet.id, "Forwarding packet to server");
                            send_raw_packet(&mut self.server_writer, &packet).await?;
                        }
                        Err(err) => {
                            error!("Failed to parse & log server-bound packet, err: {err:?}");
                            self.log_server_bound(packet.id, "Forwarding packet to server");
                            send_raw_packet(&mut self.server_writer, &packet).await?;
                        }
                    }
                }
                to_client = self.server_reader.get_packet() => {
                    let packet = match to_client {
                        Ok(p) => p,
                        Err(PacketReadError::ConnectionClosed) => {
                            warn!("Connection to server was closed");
                            return Ok(())
                        }
                        Err(e) => return Err(e.into())
                    };

                    let data_to_print = if packet.data.len() > 20 {
                        let first_part_of_data = &packet.data[..20];
                        format!("packet size: {}, first 20 bytes: {first_part_of_data:02x?}", packet.data.len())
                    } else {
                        format!("data: {:02x?}", packet.data)
                    };
                    self.log_client_bound(packet.id, format!("Packet to client, {}", data_to_print).bright_blue().to_string().as_str());
                    self.handling_packet = true;

                    match self.parse_and_log_client_bound_packet(packet.clone()).await {
                        Ok(true) => { /* The client has been dealt with */ }
                        Ok(false) => {
                            self.log_client_bound(packet.id, "Forwarding packet to client");
                            send_raw_packet(&mut self.client_writer, &packet).await?;
                        }
                        Err(err) => {
                            error!("Failed to parse & log client-bound packet, err: {err:?}");
                            self.log_client_bound(packet.id, "Forwarding packet to client");
                            send_raw_packet(&mut self.client_writer, &packet).await?;
                        }
                    }
                }
            }
            self.handling_packet = false;
        }
    }

    async fn parse_and_log_server_bound_packet(
        &mut self,
        packet: RawPacket,
    ) -> Result<bool, ProxyError> {
        let packet_id = packet.id.clone();

        self.log_server_bound(
            packet_id,
            &format!(
                "Server-bound packet with ID {} in state {:?}",
                packet_id, self.state,
            ),
        );
        match (&self.state, packet_id) {
            (&ClientState::Handshaking, 0) => {
                self.log_server_bound(packet_id, "Server-bound packet is handshake");
                let handshake = Handshake::deserialize(&mut packet.get_deserializer())?;
                self.log_server_bound(packet_id, &format!("Handshake packet: {handshake:?}"));

                let new_state = match handshake.intent.0 {
                    1 => ClientState::Status,
                    2 => ClientState::Login,
                    s => {
                        warn!("Unsupported state requested {s}");
                        return Ok(false);
                    }
                };
                self.log_server_bound(packet_id, &format!("Setting state to {new_state}"));
                self.state = new_state;
            }
            (&ClientState::Status, 0) => {
                self.log_server_bound(packet_id, "Status request");
            }
            (&ClientState::Status, 0x1) => {
                self.log_server_bound(packet_id, "Ping request");
                let ping_request = PingRequest::deserialize(&mut packet.get_deserializer())?;
                self.log_server_bound(
                    packet_id,
                    &format!("Ping request content: {ping_request:?}"),
                );
            }
            (&ClientState::Login, 0x0) => {
                self.log_server_bound(packet_id, "Login start");
                let login_start = LoginStart::deserialize(&mut packet.get_deserializer())?;
                self.log_server_bound(packet_id, &format!("Login start: {login_start:?}"));
            }
            (&ClientState::Login, 0x1) => {
                self.log_server_bound(packet_id, "Encryption response");

                // Finalize client encryption
                let encryption_response =
                    EncryptionResponse::deserialize(&mut packet.get_deserializer())?;

                let shared_secret = self
                    .key_store
                    .decrypt(encryption_response.shared_secret.inner())?;

                let verify_token = self
                    .key_store
                    .decrypt(encryption_response.verify_token.inner())?;

                if verify_token.as_slice() != [b'h', b'a', b'm'] {
                    error!("Verify token incorrect!");
                    return Err(ProxyError::InvalidPacket);
                } else {
                    self.log_server_bound(packet_id, "Verify token correct")
                }

                let shared_secret: [u8; 16] = shared_secret
                    .try_into()
                    .map_err(|_| ProxyError::InvalidPacket)?;

                self.log_server_bound(packet_id, "Enabling client encryption");
                self.client_reader.enable_encryption(&shared_secret)?;
                self.client_writer.enable_encryption(&shared_secret)?;

                return Ok(true);
            }
            (&ClientState::Login, 0x3) => {
                self.log_server_bound(packet_id, "Login Acknowledged");
                self.log_server_bound(
                    packet_id,
                    &format!("Setting state to {}", ClientState::Configuration),
                );
                self.state = ClientState::Configuration;
            }
            (&ClientState::Configuration, 0x0) => {
                self.log_server_bound(packet_id, "Client information");
                let client_info = ClientInformation::deserialize(&mut packet.get_deserializer())?;
                self.log_server_bound(packet_id, &format!("Client info: {client_info:?}"));
            }
            (&ClientState::Configuration, 0x2) => {
                self.log_server_bound(packet_id, "Plugin message");
                let plugin_message = ServerboundPluginMessage::read(packet)?;

                self.log_server_bound(packet_id, &format!("Plugin message: {plugin_message:?}"));
            }
            (&ClientState::Configuration, 0x3) => {
                self.log_server_bound(packet_id, "Finish configuration");
                self.log_server_bound(packet_id, "Transitioning to state Play");
                self.state = ClientState::Play;
            }
            (&ClientState::Configuration, 0x7) => {
                self.log_server_bound(packet_id, "Serverbound known packs");
                let known_packs =
                    ServerboundKnownPacks::deserialize(&mut packet.get_deserializer())?;
                self.log_server_bound(packet_id, &format!("Client supports: {known_packs:?}"));
            }
            (state, id) => {
                warn!("Unsupported packet ID ({id}) for state {state:?} in server-bound packets");
            }
        }

        Ok(false)
    }

    async fn parse_and_log_client_bound_packet(
        &mut self,
        packet: RawPacket,
    ) -> Result<bool, ProxyError> {
        let packet_id = packet.id.clone();

        self.log_client_bound(
            packet_id,
            &format!(
                "client-bound packet with ID {:02X} in state {:?}",
                packet_id, self.state,
            ),
        );
        match (&self.state, packet_id) {
            (&ClientState::Status, 0) => {
                self.log_client_bound(packet_id, "Status response");
                let server_status =
                    JsonString::<ServerStatus, 27512>::deserialize(&mut packet.get_deserializer())?;
                let server_status = server_status.into_inner();
                self.log_client_bound(packet_id, &format!("Server status: {server_status:?}"));
            }
            (&ClientState::Status, 0x01) => {
                self.log_client_bound(packet_id, "Pong response");
                let pong_response = PongResponse::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!("Pong response content: {pong_response:?}"),
                );
            }
            (&ClientState::Login, 0x0) => {
                self.log_client_bound(packet_id, "Login Disconnect");
                let message = String::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Disconnect reason: {message}"));
            }
            (&ClientState::Login, 0x01) => {
                self.log_client_bound(packet_id, "Encryption request");

                // Since we want to be a middle-man we need to handle this request ourselves whilst also sending a request of our own to the client.

                let incoming_encryption_request =
                    EncryptionRequest::deserialize(&mut packet.get_deserializer())?;

                let mut rng = rand::thread_rng();
                let secret: [u8; 16] = rng.r#gen();

                let encrypted_secret = KeyStore::encrypt(
                    incoming_encryption_request.public_key.inner().as_slice(),
                    secret.to_vec(),
                )?;

                let encrypted_verify_token = KeyStore::encrypt(
                    incoming_encryption_request.public_key.inner().as_slice(),
                    incoming_encryption_request.verify_token.take_inner(),
                )?;

                let encryption_response = EncryptionResponse {
                    shared_secret: PrefixedArray::new(encrypted_secret),
                    verify_token: PrefixedArray::new(encrypted_verify_token),
                };

                self.log_client_bound(packet_id, "Responding to encryption request");
                self.server_writer.write_packet(encryption_response).await?;

                self.log_client_bound(packet_id, "Enabling server encryption");
                self.server_writer.enable_encryption(&secret)?;
                self.server_reader.enable_encryption(&secret)?;

                self.log_client_bound(packet_id, "Sending encryption request to client");
                let outgoing_encryption_request =
                    EncryptionRequest::new(self.key_store.get_der_public_key());

                self.client_writer
                    .write_packet(outgoing_encryption_request)
                    .await?;

                return Ok(true);
            }
            (&ClientState::Login, 0x2) => {
                self.log_client_bound(packet_id, "Login successful");
            }
            (&ClientState::Login, 0x3) => {
                self.log_client_bound(packet_id, "Enable compression");
                todo!("Compression is not yet supported")
            }
            (&ClientState::Configuration, 0x0) => {
                self.log_client_bound(packet_id, "Cookie request");
                let key = String::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Cookie request key: \"{key}\""));
            }
            (&ClientState::Configuration, 0x1) => {
                self.log_client_bound(packet_id, "Clientbound Plugin Message");
                let clientbound_plugin_message = ClientboundPluginMessage::read(packet)?;
                self.log_client_bound(
                    packet_id,
                    &format!("Plugin message: {clientbound_plugin_message:?}"),
                );
            }
            (&ClientState::Configuration, 0x3) => {
                self.log_client_bound(packet_id, "Finish configuration");
            }
            (&ClientState::Configuration, 0x4) => {
                self.log_client_bound(packet_id, "Clientbound keep alive (configuration)");
                let keep_alive = ClientboundKeepAlive::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Keep alive request {keep_alive:?}"));
            }
            (&ClientState::Configuration, 0x7) => {
                self.log_client_bound(packet_id, "Registry data packet");
                let registry_data = RegistryData::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Registry data: {registry_data:?}"));
            }
            (&ClientState::Configuration, 0xC) => {
                self.log_client_bound(packet_id, "Feature flags");
                let feature_flags = FeatureFlags::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Feature flags: {feature_flags:?}"));
            }
            (&ClientState::Configuration, 0xD) => {
                self.log_client_bound(packet_id, "Update tags");
                let update_tags = UpdateTags::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!(
                        "Update tags for registries: [{}]",
                        update_tags
                            .tagged_registries
                            .inner()
                            .iter()
                            .map(|registry| format!("{}, ", registry.registry))
                            .collect::<String>()
                            .trim_end_matches(", ")
                    ),
                );
            }
            (&ClientState::Configuration, 0xE) => {
                self.log_client_bound(packet_id, "Clientbound known packs");
                let known_packs =
                    ClientboundKnownPacks::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Server packs: {known_packs:?}"));
            }
            (&ClientState::Play, 0x0a) => {
                self.log_client_bound(packet_id, "Change difficulty");
                let change_difficulty =
                    ChangeDifficulty::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!("Change difficulty packet: {change_difficulty:?}"),
                );
            }
            (&ClientState::Play, 0x30) => {
                self.log_client_bound(packet_id, "Login (play)");
                let login = Login::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(packet_id, &format!("Login packet: {login:?}"));
            }
            (&ClientState::Play, 0x3E) => {
                self.log_client_bound(packet_id, "Player abilities");
                let player_abilities =
                    PlayerAbilities::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!("Player abilities: {player_abilities:?}"),
                );
            }
            (&ClientState::Play, 0x67) => {
                self.log_client_bound(packet_id, "Set held item");
                let set_held_item = SetHeldItem::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!("Set held item packet: {set_held_item:?}"),
                );
            }
            (&ClientState::Play, 0x83) => {
                self.log_client_bound(packet_id, "Update recipes");
                let update_recipes = UpdateRecipes::deserialize(&mut packet.get_deserializer())?;
                self.log_client_bound(
                    packet_id,
                    &format!("Update recipes packet: {update_recipes:?}"),
                );
            }
            (state, id) => {
                warn!(
                    "Unsupported packet ID ({id:02x}) for state {state:?} in client-bound packets"
                );
            }
        }

        Ok(false)
    }

    fn log_client_bound(&self, packet_id: i32, msg: &str) {
        let space = if self.handling_packet { "    > " } else { "" };
        info!(
            "[{} > {}]:[{}]:[{}] :: {space} {msg}",
            "S".green(),
            "C".blue(),
            self.state,
            format!("0x{packet_id:02x}").purple(),
        )
    }

    fn log_server_bound(&self, packet_id: i32, msg: &str) {
        let space = if self.handling_packet { "    > " } else { "" };
        info!(
            "[{} > {}]:[{}]:[{}] :: {space} {msg}",
            "C".blue(),
            "S".green(),
            self.state,
            format!("0x{packet_id:02x}").purple(),
        )
    }
}

// TODO: Unnecessary allocations, should probably just implement AsyncWrite for NetworkWriter but who can be arsed?
async fn send_raw_packet(
    writer: &mut NetworkWriter<BufWriter<OwnedWriteHalf>>,
    packet: &RawPacket,
) -> Result<(), ProxyError> {
    let mut buffer = Vec::new();

    let id = VarInt::from(packet.id);
    buffer.write_var_int(&id)?;
    io::copy(&mut Cursor::new(packet.data.clone()), &mut buffer)?;

    let total_length: VarInt = buffer.len().try_into().map_err(|err| {
        error!("Packet length received was too large? error: {err:?}");
        ProxyError::InvalidPacket
    })?;

    let mut final_buffer = Vec::new();
    total_length.encode(&mut final_buffer)?;
    io::copy(&mut Cursor::new(buffer), &mut final_buffer)?;

    writer.write_data(final_buffer).await?;

    Ok(())
}
