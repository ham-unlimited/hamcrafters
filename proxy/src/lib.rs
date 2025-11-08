#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling proxying to another Minecraft server.

use std::io::{self, Cursor};

use log::{error, info, warn};
use mc_coms::{
    client_state::ClientState,
    codec::{json_string::JsonString, var_int::VarInt},
    messages::{
        clientbound::status::{pong_response::PongResponse, status_response::ServerStatus},
        serverbound::{handshaking::handshake::Handshake, status::ping_request::PingRequest},
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    ser::{NetworkWriteExt, ReadingError, WritingError},
};
use serde::Deserialize;
use tokio::{
    io::{AsyncWrite, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use tokio::io::AsyncWriteExt;

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
}

/// Handling connection for the proxy.
pub struct ProxyHandler {
    client_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    // client_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    client_writer: OwnedWriteHalf,

    server_reader: NetworkReader<BufReader<OwnedReadHalf>>,
    // server_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
    server_writer: OwnedWriteHalf,

    state: ClientState,
}

impl ProxyHandler {
    /// Creates a [ProxyHandler] from the provided [stream].
    pub async fn new(stream: TcpStream, target: &str) -> Result<Self, ProxyError> {
        let (client_reader, client_writer) = stream.into_split();
        let client_reader = NetworkReader::new(BufReader::new(client_reader));
        // let client_writer = NetworkWriter::new(BufWriter::new(client_writer));

        let out_stream = TcpStream::connect(target)
            .await
            .map_err(ProxyError::FailedStartingServerComs)?;
        info!("Connection setup to {target}");

        let (server_reader, server_writer) = out_stream.into_split();
        let server_reader = NetworkReader::new(BufReader::new(server_reader));
        // let server_writer = NetworkWriter::new(BufWriter::new(server_writer));

        Ok(ProxyHandler {
            client_reader,
            client_writer,
            server_reader,
            server_writer,
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

                    info!("Packet to server {packet:?}");
                    if let Err(err) = self.parse_and_log_server_bound_packet(packet.clone()) {
                        error!("Failed to parse&log server-bound packet, err: {err:?}");
                    }
                    send_raw_packet(&mut self.server_writer, &packet).await?;
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
                    info!("Packet to client {packet:?}");
                    if let Err(err) = self.parse_and_log_client_bound_packet(packet.clone()) {
                        error!("Failed to parse&log client-bound packet, err: {err:?}");
                    }
                    send_raw_packet(&mut self.client_writer, &packet).await?;
                }
            }
        }
    }

    fn parse_and_log_server_bound_packet(&mut self, packet: RawPacket) -> Result<(), ProxyError> {
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
                        return Ok(());
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
            (state, id) => {
                warn!("Unsupported packet ID ({id}) for state {state:?} in server-bound packets")
            }
        }

        Ok(())
    }

    fn parse_and_log_client_bound_packet(&mut self, packet: RawPacket) -> Result<(), ProxyError> {
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
            (state, id) => {
                warn!("Unsupported packet ID ({id}) for state {state:?} in client-bound packets");
            }
        }

        Ok(())
    }
}

async fn send_raw_packet<W: AsyncWrite + Unpin>(
    writer: &mut W,
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

    total_length.encode_async(writer).await?;
    writer.write_all(&buffer).await?;

    Ok(())
}
