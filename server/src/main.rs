use eyre::Context;
use log::info;

use mc_coms::{
    messages::{
        clientbound::status::{pong_response::PongResponse, status_response::StatusResponse},
        serverbound::{handshaking::handshake::Handshake, status::ping_request::PingRequest},
    },
    packet_reader::{NetworkReader, PacketReadError, RawPacket},
    packet_writer::NetworkWriter,
};
use serde::Deserialize;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

const SUPPORTED_MINECRAFT_PROTOCOL_VERSION: usize = 773;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().wrap_err("Failed to read dotenv")?;
    color_eyre::install()?;
    pretty_env_logger::init();

    let host = "127.0.0.1:22211";
    let listener = TcpListener::bind(host)
        .await
        .wrap_err("Listening on {host}")?;

    info!("Server listening on {host}");

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .wrap_err("Failed to receive incoming connection")?;

        info!("Receiving connection from {:?}", addr);

        let mut handler = ClientHandler::new(stream);
        handler
            .run()
            .await
            .wrap_err("Error occurred during running of program")?;
    }
}

#[derive(Debug, Clone)]
enum ClientState {
    Handshaking,
    Status,
    Login,
}

struct ClientHandler {
    reader: NetworkReader<BufReader<OwnedReadHalf>>,
    state: ClientState,
    network_writer: NetworkWriter<BufWriter<OwnedWriteHalf>>,
}

impl ClientHandler {
    #[must_use]
    fn new(stream: TcpStream) -> Self {
        let (r, w) = stream.into_split();

        let reader = NetworkReader::new(BufReader::new(r));
        let writer = NetworkWriter::new(BufWriter::new(w));

        Self {
            reader,
            state: ClientState::Handshaking,
            network_writer: writer,
        }
    }

    async fn run(&mut self) -> eyre::Result<()> {
        // let mut proxy_handler = ProxyHandler::new();
        loop {
            let packet = match self.reader.get_packet().await {
                Ok(s) => s,
                Err(PacketReadError::ConnectionClosed) => return Ok(()),
                Err(e) => Err(e).wrap_err("Failed to read packet")?,
            };

            info!("Got new packet: {packet:?}");

            match self.state {
                ClientState::Handshaking => self
                    .handle_handshake_packet(packet)
                    .wrap_err("Failed to handle handshake packet")?,
                ClientState::Status => self
                    .handle_status_packet(packet)
                    .await
                    .wrap_err("Failed to handle status packet")?,
                ClientState::Login => todo!("Login not yet supported"),
            }
        }
    }

    fn handle_handshake_packet(&mut self, packet: RawPacket) -> eyre::Result<()> {
        match packet.id {
            0x0 => {
                let handshake = Handshake::deserialize(&mut packet.get_deserializer())
                    .wrap_err("Failed to deserialize handshake")?;

                info!("Received handshake request: {handshake:?}");

                if handshake.protocol_version.0 as usize != SUPPORTED_MINECRAFT_PROTOCOL_VERSION {
                    println!(
                        "Unsupported protocol version {}",
                        handshake.protocol_version.0
                    );
                    eyre::bail!("Illegal protocol version");
                }

                self.state = match handshake.intent.0 {
                    1 => ClientState::Status,
                    2 => ClientState::Login,
                    3 => unimplemented!("ClientState::Transfer?"),
                    s => panic!("Illegal client state requested {s}"),
                };
                info!("New server state {:?}", self.state);
            }
            id => eyre::bail!("Unsupported packet ID for state handshake {id}"),
        }

        Ok(())
    }

    async fn handle_status_packet(&mut self, packet: RawPacket) -> eyre::Result<()> {
        match packet.id {
            0x0 => {
                info!("Got status request");
                let status_response = StatusResponse::default();

                self.network_writer
                    .write_packet(status_response)
                    .await
                    .wrap_err("Failed to send status response")?;

                info!("Responded to status request");
            }
            0x1 => {
                let ping_request = PingRequest::deserialize(&mut packet.get_deserializer())
                    .wrap_err("Failed to deserialize ping request")?;

                info!("Got status ping with request: {ping_request:?}");

                let pong_response: PongResponse = ping_request.into();

                self.network_writer
                    .write_packet(pong_response)
                    .await
                    .wrap_err("Failed to write pong_response")?;

                info!("Responded to ping request")
            }
            id => eyre::bail!("Unsupported packet ID for state handshake {id}"),
        }
        Ok(())
    }
}
