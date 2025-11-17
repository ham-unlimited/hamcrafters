#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling proxying to another Minecraft server.

use std::{
    io::{self, Cursor},
    str::FromStr,
};

use log::{error, info, warn};
use mc_coms::{
    client_state::ClientState,
    codec::{json_string::JsonString, prefixed_array::PrefixedArray, var_int::VarInt},
    key_store::{EncryptionError, KeyStore},
    messages::{
        clientbound::{
            login::{
                encryption_request::EncryptionRequest,
                login_success::{GameProfile, LoginSuccess},
            },
            status::{pong_response::PongResponse, status_response::ServerStatus},
        },
        serverbound::{
            handshaking::handshake::Handshake, login::encryption_response::EncryptionResponse,
            status::ping_request::PingRequest,
        },
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    packet_writer::{NetworkWriter, PacketWriteError},
    ser::{NetworkWriteExt, ReadingError, WritingError},
};
use rand::Rng;
use serde::Deserialize;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};
use uuid::Uuid;

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
}

/// Handling connection for the proxy.
pub struct ProxyHandler<'key> {
    client_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    client_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    server_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    server_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    key_store: &'key KeyStore,
    state: ClientState,
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
        })
    }

    /// Start the [ProxyHandler] and handle connections.
    pub async fn run(&mut self) -> Result<(), ProxyError> {
        loop {
            tokio::select! {
                to_server = self.client_reader.get_packet() => {
                    let packet = match to_server {
                        Ok(p) => p,
                        Err(PacketReadError::ConnectionClosed) => {
                            info!("Connection to client was closed");
                            return Ok(())
                        }
                        Err(e) => return Err(e.into())
                    };

                    info!("Packet to server {packet:02x?}");

                    match self.parse_and_log_server_bound_packet(packet.clone()).await {
                        Ok(true) => { /* The server has been dealt with */ }
                        Ok(false) => {
                            send_raw_packet(&mut self.server_writer, &packet).await?;
                        }
                        Err(err) => {
                            error!("Failed to parse & log server-bound packet, err: {err:?}");
                            send_raw_packet(&mut self.server_writer, &packet).await?;
                        }
                    }
                }
                to_client = self.server_reader.get_packet() => {
                    let packet = match to_client {
                        Ok(p) => p,
                        Err(PacketReadError::ConnectionClosed) => {
                            info!("Connection to server was closed");
                            return Ok(())
                        }
                        Err(e) => return Err(e.into())
                    };
                    info!("Packet to client {packet:02x?}");
                    match self.parse_and_log_client_bound_packet(packet.clone()).await {
                        Ok(true) => { /* The client has been dealt with */ }
                        Ok(false) => {
                            send_raw_packet(&mut self.client_writer, &packet).await?;
                        }
                        Err(err) => {
                            error!("Failed to parse & log client-bound packet, err: {err:?}");
                            send_raw_packet(&mut self.client_writer, &packet).await?;
                        }
                    }
                }
            }
        }
    }

    async fn parse_and_log_server_bound_packet(
        &mut self,
        packet: RawPacket,
    ) -> Result<bool, ProxyError> {
        info!(
            "Server-bound packet with ID {} in state {:?}",
            packet.id, self.state
        );
        match (&self.state, packet.id) {
            (&ClientState::Handshaking, 0) => {
                info!("Server-bound packet is handshake");
                let handshake = Handshake::deserialize(&mut packet.get_deserializer())?;
                info!("Handshake packet: {handshake:?}");
                self.state = match handshake.intent.0 {
                    1 => ClientState::Status,
                    2 => ClientState::Login,
                    s => {
                        warn!("Unsupported state requested {s}");
                        return Ok(false);
                    }
                };
            }
            (&ClientState::Status, 0) => {
                info!("Status request");
            }
            (&ClientState::Status, 0x1) => {
                info!("Ping request");
                let ping_request = PingRequest::deserialize(&mut packet.get_deserializer())?;
                info!("Ping request content: {ping_request:?}");
            }
            (&ClientState::Login, 0x1) => {
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
                    info!("Verify token correct")
                }

                let shared_secret: [u8; 16] = shared_secret
                    .try_into()
                    .map_err(|_| ProxyError::InvalidPacket)?;

                info!("Enabling client encryption");
                self.client_reader.enable_encryption(&shared_secret)?;
                self.client_writer.enable_encryption(&shared_secret)?;

                let id = Uuid::from_str("00002a4a-0000-1000-8000-00805f9b34fb")?;

                let login_success = LoginSuccess {
                    profile: GameProfile {
                        uuid: id,
                        username: "Pepe".into(),
                        properties: PrefixedArray::empty(),
                    },
                };

                info!("Responding with login success");
                self.client_writer.write_packet(login_success).await?;

                // TODO: Handle server encryption.

                return Ok(true);
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
        info!(
            "client-bound packet with ID {} in state {:?}",
            packet.id, self.state
        );
        match (&self.state, packet.id) {
            (&ClientState::Status, 0) => {
                info!("Status response");
                let server_status =
                    JsonString::<ServerStatus, 27512>::deserialize(&mut packet.get_deserializer())?;
                let server_status = server_status.into_inner();
                info!("Server status: {server_status:?}");
            }
            (&ClientState::Status, 0x01) => {
                info!("Pong response");
                let pong_response = PongResponse::deserialize(&mut packet.get_deserializer())?;
                info!("Pong response content: {pong_response:?}");
            }
            (&ClientState::Login, 0x01) => {
                info!("Encryption request");

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

                info!("Responding to encryption request");
                self.server_writer.write_packet(encryption_response).await?;

                info!("Enabling server encryption");
                self.server_writer.enable_encryption(&secret)?;
                self.server_reader.enable_encryption(&secret)?;

                info!("Sending encryption request to client");
                let outgoing_encryption_request =
                    EncryptionRequest::new(self.key_store.get_der_public_key());

                self.client_writer
                    .write_packet(outgoing_encryption_request)
                    .await?;

                return Ok(true);
            }
            (state, id) => {
                warn!("Unsupported packet ID ({id}) for state {state:?} in client-bound packets");
            }
        }

        Ok(false)
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
