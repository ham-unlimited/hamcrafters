use std::{
    io::{self, Cursor, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use chrono::{Duration, Utc};
use log::info;
use serde::Deserialize;

use crate::{
    codec::var_int::VarInt,
    coms::{NetworkReadExt, deserialize::Deserializer},
    serial::PacketWrite,
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
        let mut destination =
            TcpStream::connect("85.24.227.95:25565").expect("Failed to connect to upstream server");
        destination
            .set_write_timeout(Some(std::time::Duration::from_secs(30)))
            .expect("Failed to set write timeout on outgoing client");
        destination
            .set_read_timeout(Some(std::time::Duration::from_secs(30)))
            .expect("Failed to set read timeout");

        info!(
            "Connected to remote server using sourceport: {}",
            destination.local_addr().expect("Local addr").port()
        );

        let mut buffer = Vec::new();
        let mut response_buffer = Vec::new();

        loop {
            // TODO: Security...
            let raw_length = self
                .stream
                .get_var_int()
                .expect("Failed to read packet length");
            let length = raw_length.0 as usize;
            info!("Got new package of length: {length}");

            // Resize buffer if needed
            if buffer.len() < length {
                buffer.resize(length, 0);
            }

            self.stream
                .read_exact(&mut buffer[..length])
                .expect("Failed to read buffer");

            // Is either 0 => Status request, or 1 => Ping request.
            let next_packet_id = self
                .stream
                .get_var_int()
                .expect("Failed to read next packet ID");

            let mut cursor = Cursor::new(&buffer[..length]);

            /* Proxy */
            let mut write_cursor = cursor.clone();

            let mut output_buffer = Vec::new();
            raw_length
                .encode(&mut output_buffer)
                .expect("Failed to write length to output buffer");
            io::copy(&mut write_cursor, &mut output_buffer)
                .expect("Failed to write data to output buffer");

            destination
                .write_all(&output_buffer)
                .expect("Failed to send message to remote server");

            next_packet_id
                .encode(&mut destination)
                .expect("Failed to write extra data");

            destination
                .flush()
                .expect("Failed to flush data to upstream");

            // TODO: Calculate our own response.
            self.handle_packet(&mut cursor);

            let exit_time = Utc::now() + Duration::seconds(30);
            let response_length = loop {
                if let Ok(response_length) = destination.get_var_int() {
                    break response_length.0 as usize;
                }

                thread::sleep(std::time::Duration::from_millis(500));

                if Utc::now() > exit_time {
                    panic!("Timeout waiting for response from upstream");
                }
            };

            info!("Got new response of length: {response_length}");

            // Resize buffer if needed
            if response_buffer.len() < response_length {
                response_buffer.resize(response_length, 0);
            }

            destination
                .read_exact(&mut response_buffer[..response_length])
                .expect("Failed to read buffer");

            info!("Response bytes: {:?}", &response_buffer[..response_length]);
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
