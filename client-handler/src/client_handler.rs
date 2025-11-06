use log::info;
use mc_coms::{
    SUPPORTED_MINECRAFT_PROTOCOL_VERSION,
    client_state::ClientState,
    messages::{
        clientbound::status::{pong_response::PongResponse, status_response::StatusResponse},
        serverbound::{handshaking::handshake::Handshake, status::ping_request::PingRequest},
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    packet_writer::NetworkWriter,
};
use serde::Deserialize;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

use crate::client_error::ClientError;

/// Handles communication between the server and a specific Minecraft client.
pub struct ClientHandler<S: AsyncRead + AsyncWrite + Send + Unpin> {
    stream: S,
    state: ClientState,
}

impl<S: AsyncRead + AsyncWrite + Send + Unpin> ClientHandler<S> {
    /// Creates a new [ClientHandler] from the provided stream.
    #[must_use]
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            state: ClientState::Handshaking,
        }
    }

    /// Starts listening for & handling packets from the server.
    pub async fn run(&mut self) -> Result<(), ClientError> {
        loop {
            let packet = match self.read_packet().await {
                Ok(p) => p,
                Err(PacketReadError::ConnectionClosed) => return Ok(()),
                Err(err) => return Err(err.into()),
            };

            info!("Got new packet: {packet:?}");

            match self.state {
                ClientState::Handshaking => self.handle_handshake_packet(packet)?,
                ClientState::Status => self.handle_status_packet(packet).await?,
                ClientState::Login => todo!("Login not yet supported"),
            }
        }
    }

    async fn read_packet(&mut self) -> Result<RawPacket, PacketReadError> {
        let mut reader = NetworkReader::new(&mut self.stream);
        reader.get_packet().await
    }

    async fn write_packet<P: mc_coms::messages::McPacket + serde::Serialize>(
        &mut self,
        packet: P,
    ) -> Result<(), mc_coms::packet_writer::PacketWriteError> {
        let mut writer = NetworkWriter::new(&mut self.stream);
        writer.write_packet(packet).await
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

                self.write_packet(status_response).await?;

                info!("Responded to status request");
            }
            0x1 => {
                let ping_request = PingRequest::deserialize(&mut packet.get_deserializer())?;

                info!("Got status ping with request: {ping_request:?}");

                let pong_response: PongResponse = ping_request.into();

                self.write_packet(pong_response).await?;

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
}

// Keep backward compatibility with TcpStream
impl ClientHandler<TcpStream> {
    /// Creates a new [ClientHandler] from a [TcpStream].
    #[must_use]
    pub fn new_tcp(stream: TcpStream) -> Self {
        Self::new(stream)
    }
}
