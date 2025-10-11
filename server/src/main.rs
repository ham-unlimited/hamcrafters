use client_handler::client_handler::ClientHandler;
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
