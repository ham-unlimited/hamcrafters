use log::info;
use mc_coms::{
    SUPPORTED_MINECRAFT_PROTOCOL_VERSION,
    client_state::ClientState,
    messages::{
        clientbound::{
            login::encryption_request::EncryptionRequest,
            status::{pong_response::PongResponse, status_response::StatusResponse},
        },
        serverbound::{
            handshaking::handshake::Handshake,
            login::{encryption_response, login_start::LoginStart},
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

use crate::client_error::ClientError;

/// Handles communication between the server and a specific Minecraft client.
pub struct ClientHandler {
    reader: NetworkReader<BufReader<OwnedReadHalf>>,
    state: ClientState,
    public_key: String,
    private_key: String,
    network_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
}

impl ClientHandler {
    /// Creates a new [ClientHandler] from the provided [TcpStream].
    #[must_use]
    pub fn new(stream: TcpStream, public_key: String, private_key: String) -> Self {
        let (r, w) = stream.into_split();

        let reader = NetworkReader::new(BufReader::new(r));
        let writer = NetworkWriter::new(BufWriter::new(w));

        Self {
            reader,
            state: ClientState::Handshaking,
            public_key,
            private_key,
            network_writer: writer,
        }
    }

    /// Starts listening for & handling packets from the server.
    pub async fn run(&mut self) -> Result<(), ClientError> {
        loop {
            let packet = match self.reader.get_packet().await {
                Ok(p) => p,
                Err(PacketReadError::ConnectionClosed) => return Ok(()),
                Err(err) => return Err(err.into()),
            };

            info!("Got new packet: {packet:?}");

            match self.state {
                ClientState::Handshaking => self.handle_handshake_packet(packet)?,
                ClientState::Status => self.handle_status_packet(packet).await?,
                ClientState::Login => self.handle_login_packet(packet).await?,
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
                let encryption_request = EncryptionRequest::new(&self.public_key);

                info!("Encryption request: {encryption_request:?}");

                self.network_writer.write_packet(encryption_request).await?;

                info!("Responded to encryption request");
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
