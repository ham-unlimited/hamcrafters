use std::{
    io::{Cursor, Read},
    net::{TcpListener, TcpStream},
};

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
        let mut handler = ClientHandler::new(stream);
        handler.run();
    }
}

#[derive(Debug, Clone)]
enum ClientState {
    New,
    Status,
    Login,
}

struct ClientHandler {
    stream: TcpStream,
    state: ClientState,
}

impl ClientHandler {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            state: ClientState::New,
        }
    }

    fn run(&mut self) {
        let source_ip = self.stream.peer_addr().expect("Failed to read peer addr");
        info!("Receiving connection from {:?}", source_ip);

        let mut buffer = Vec::new();

        loop {
            // TODO: Security...
            let length = self
                .stream
                .get_var_int()
                .expect("Failed to read packet length");
            let length = length.0 as usize;
            info!("Got new package of length: {length}");

            // Resize buffer if needed
            if buffer.len() < length {
                buffer.resize(length, 0);
            }

            self.stream
                .read_exact(&mut buffer[..length])
                .expect("Failed to read buffer");

            let mut cursor = Cursor::new(&buffer[..length]);

            self.handle_packet(&mut cursor);
        }
    }

    fn handle_packet<T>(&mut self, cursor: &mut Cursor<T>)
    where
        T: AsRef<[u8]>,
    {
        let packet_id: VarInt = cursor.get_var_int().expect("Failed to read packet ID");
        let packet_id = packet_id.0 as usize;
        let mut deserializer = Deserializer::new(cursor);

        match (&self.state, packet_id) {
            (ClientState::New, 0) => {
                let handshake = Handshake::deserialize(&mut deserializer)
                    .expect("Failed to deserialize Deserializer");

                if handshake.protocol_version.0 as usize != SUPPORTED_MINECRAFT_PROTOCOL_VERSION {
                    println!(
                        "Unsupported protocol version {}",
                        handshake.protocol_version.0
                    );
                    panic!("Illegal protocol version");
                }

                log::info!("ID: {:X}, Packet: {:?}", packet_id, handshake);
            }
            _ => unimplemented!(
                "Unimplemented packet of ID {packet_id:?} during state {:?}",
                self.state
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Handshake {
    protocol_version: VarInt,
    server_address: String,
    server_port: u16,
    intent: VarInt, // 1 = Status, 2 = Login, 3 = Transfer(?)
}
