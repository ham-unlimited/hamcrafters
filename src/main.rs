use std::net::{TcpListener, TcpStream};

use log::info;
use serde::Deserialize;

use crate::{
    codec::var_int::VarInt,
    coms::{NetworkReadExt, deserialize::Deserializer},
};

pub mod codec;
pub mod coms;
pub mod serial;

const SUPPORTED_MINECRAFT_PROTOCOL_VERSION: usize = 773;

fn main() {
    pretty_env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:22211").expect("Failed to setup server");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to read stream");
        handle_client(stream);
    }
}

fn handle_client(mut stream: TcpStream) {
    let source_ip = stream.peer_addr().expect("Failed to read peer addr");
    info!("Receiving connection from {:?}", source_ip);

    loop {
        let length = stream.get_var_int().expect("Failed to read packet length");
        let packet_id = stream.get_var_int().expect("Failed to read packet ID");
        let mut deserializer = Deserializer::new(stream);

        let handshake =
            Handshake::deserialize(&mut deserializer).expect("Failed to deserialize Deserializer");

        // let buf_len = (length.0 as usize) - packet_id.written_size();

        if handshake.protocol_version.0 as usize != SUPPORTED_MINECRAFT_PROTOCOL_VERSION {
            println!(
                "Unsupported protocol version {}",
                handshake.protocol_version.0
            );
            // TODO: Close?
            break;
        }

        log::info!(
            "Length: {}, ID: {:X}, Packet: {:?}",
            length.0,
            packet_id.0,
            handshake,
        );

        break;
    }
}

#[derive(Debug, Deserialize)]
struct Handshake {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    intent: VarInt, // 1 = Status, 2 = Login, 3 = Transfer(?)
}
