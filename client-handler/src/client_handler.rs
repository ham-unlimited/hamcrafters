use std::str::FromStr;

use log::{error, info};
use mc_coms::{
    SUPPORTED_MINECRAFT_PROTOCOL_VERSION,
    client_state::ClientState,
    codec::prefixed_array::PrefixedArray,
    key_store::KeyStore,
    messages::{
        clientbound::{
            login::{
                encryption_request::EncryptionRequest,
                login_success::{GameProfile, LoginSuccess},
            },
            status::{pong_response::PongResponse, status_response::StatusResponse},
        },
        serverbound::{
            configuration::{
                client_information::ClientInformation,
                serverbound_plugin_message::ServerboundPluginMessage,
            },
            handshaking::handshake::Handshake,
            login::encryption_response::EncryptionResponse,
            status::ping_request::PingRequest,
        },
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    packet_writer::NetworkWriter,
};
use serde::Deserialize;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use uuid::Uuid;

use crate::client_error::ClientError;

/// Handles communication between the server and a specific Minecraft client.
pub struct ClientHandler<'key> {
    state: ClientState,
    key_store: &'key KeyStore,
    network_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    network_reader: NetworkReader<BufReader<OwnedReadHalf>>,
}

impl<'key> ClientHandler<'key> {
    /// Creates a new [ClientHandler] from the provided [TcpStream].
    #[must_use]
    pub fn new(stream: TcpStream, key_store: &'key KeyStore) -> Self {
        let (r, w) = stream.into_split();

        let reader = NetworkReader::new(BufReader::new(r));
        let writer = NetworkWriter::new(BufWriter::new(w));

        Self {
            state: ClientState::Handshaking,
            key_store,
            network_reader: reader,
            network_writer: writer,
        }
    }

    /// Starts listening for & handling packets from the server.
    pub async fn run(&mut self) -> Result<(), ClientError> {
        loop {
            let packet = match self.network_reader.get_packet().await {
                Ok(p) => p,
                Err(PacketReadError::ConnectionClosed) => return Ok(()),
                Err(err) => return Err(err.into()),
            };

            info!("Got new packet (state: {:?}): {packet:02x?}", self.state);

            match self.state {
                ClientState::Handshaking => self.handle_handshake_packet(packet)?,
                ClientState::Status => self.handle_status_packet(packet).await?,
                ClientState::Login => self.handle_login_packet(packet).await?,
                ClientState::Configuration => self.handle_configuration_packet(packet).await?,
            }
        }
    }

    fn handle_handshake_packet(&mut self, packet: RawPacket) -> Result<(), ClientError> {
        match packet.id {
            0x0 => {
                let handshake = Handshake::deserialize(&mut packet.get_deserializer())?;

                info!("Received handshake request: {handshake:?}");

                if handshake.protocol_version.0 as usize != SUPPORTED_MINECRAFT_PROTOCOL_VERSION {
                    println!(
                        "Unsupported protocol version {}",
                        handshake.protocol_version.0
                    );
                    return Err(ClientError::InvalidProtocolVersion {
                        received_version: handshake.protocol_version.0 as usize,
                        supported_version: SUPPORTED_MINECRAFT_PROTOCOL_VERSION,
                    });
                }

                self.state = match handshake.intent.0 {
                    1 => ClientState::Status,
                    2 => ClientState::Login,
                    3 => unimplemented!("ClientState::Transfer?"),
                    s => panic!("Illegal client state requested {s}"),
                };
                info!("New server state {:?}", self.state);
            }
            id => {
                return Err(ClientError::UnsupportedPacketId {
                    packet_id: id,
                    state: ClientState::Handshaking,
                });
            }
        }

        Ok(())
    }

    async fn handle_status_packet(&mut self, packet: RawPacket) -> Result<(), ClientError> {
        match packet.id {
            0x0 => {
                info!("Got status request");
                let status_response = StatusResponse::default();

                info!("Status response: {status_response:?}");

                self.network_writer.write_packet(status_response).await?;

                info!("Responded to status request");
            }
            0x1 => {
                let ping_request = PingRequest::deserialize(&mut packet.get_deserializer())?;

                info!("Got status ping with request: {ping_request:?}");

                let pong_response: PongResponse = ping_request.into();

                self.network_writer.write_packet(pong_response).await?;

                info!("Responded to ping request")
            }
            id => {
                return Err(ClientError::UnsupportedPacketId {
                    packet_id: id,
                    state: ClientState::Status,
                });
            }
        }
        Ok(())
    }

    async fn handle_login_packet(&mut self, packet: RawPacket) -> Result<(), ClientError> {
        match packet.id {
            0x0 => {
                info!("Got login start request");

                info!("Creating encryption request");
                let encryption_request =
                    EncryptionRequest::new(self.key_store.get_der_public_key());

                info!("Encryption request: {encryption_request:02x?}");

                info!("Verify token: {:02x?}", encryption_request.verify_token);

                self.network_writer.write_packet(encryption_request).await?;

                info!("Responded to encryption request");
            }
            0x1 => {
                info!("Got encryption response");

                let encryption_response =
                    EncryptionResponse::deserialize(&mut packet.get_deserializer())?;

                info!("Encryption response: {encryption_response:02x?}");

                let shared_secret = self
                    .key_store
                    .decrypt(encryption_response.shared_secret.inner())?;

                let verify_token = self
                    .key_store
                    .decrypt(encryption_response.verify_token.inner())?;

                if verify_token.as_slice() != [b'h', b'a', b'm'] {
                    error!("Verify token incorrect!");
                    return Err(ClientError::InvalidVerifyToken);
                } else {
                    info!("Verify token correct")
                }

                let shared_secret: [u8; 16] = shared_secret
                    .try_into()
                    .map_err(|_| ClientError::InvalidSharedSecret)?;

                self.network_writer.enable_encryption(&shared_secret)?;
                self.network_reader.enable_encryption(&shared_secret)?;

                let id = Uuid::from_str("00002a4a-0000-1000-8000-00805f9b34fb")?;

                let login_success = LoginSuccess {
                    profile: GameProfile {
                        uuid: id,
                        username: "Pepe".into(),
                        properties: PrefixedArray::empty(),
                    },
                };

                info!("Responding with login success");

                self.network_writer.write_packet(login_success).await?;
            }
            0x3 => {
                info!("Login acknowledged received");
                self.state = ClientState::Configuration;
            }
            id => {
                return Err(ClientError::UnsupportedPacketId {
                    packet_id: id,
                    state: ClientState::Login,
                });
            }
        }
        Ok(())
    }

    async fn handle_configuration_packet(&mut self, packet: RawPacket) -> Result<(), ClientError> {
        match packet.id {
            0x0 => {
                info!("Client configuration message");

                let client_info = ClientInformation::deserialize(&mut packet.get_deserializer())?;

                info!("Client info: {client_info:?}");
            }
            0x2 => {
                info!("Received plugin message");

                let plugin_message =
                    ServerboundPluginMessage::deserialize(&mut packet.get_deserializer())?;

                info!(
                    "Message in channel: {} of length TODO",
                    plugin_message.channel,
                );
            }
            id => {
                return Err(ClientError::UnsupportedPacketId {
                    packet_id: id,
                    state: ClientState::Configuration,
                });
            }
        }

        Ok(())
    }
}
