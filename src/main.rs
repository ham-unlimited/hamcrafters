use std::{
    io::{self, Cursor, Read, Write},
    net::{TcpListener, TcpStream},
};

use log::info;
use serde::Deserialize;

use crate::{
    codec::var_int::VarInt,
    coms::{NetworkReadExt, deserialize::Deserializer},
    messages::serverbound::handshake::Handshake,
};

pub mod codec;
pub mod coms;
pub mod messages;
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

struct ProxyHandler {
    upstream: TcpStream,
    buffer: Vec<u8>,
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

        let mut proxy_handler = ProxyHandler::new();

        let mut buffer = Vec::new();

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

            proxy_handler.proxy_request(
                raw_length,
                cursor.clone(),
                next_packet_id,
                &mut self.stream,
            );

            // TODO: Calculate our own response.
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

impl ProxyHandler {
    fn new() -> Self {
        let upstream =
            TcpStream::connect("85.24.227.95:25565").expect("Failed to connect to upstream server");
        upstream
            .set_write_timeout(Some(std::time::Duration::from_secs(30)))
            .expect("Failed to set write timeout on outgoing client");
        upstream
            .set_read_timeout(Some(std::time::Duration::from_secs(30)))
            .expect("Failed to set read timeout");

        info!(
            "Connected to remote server using sourceport: {}",
            upstream.local_addr().expect("Local addr").port()
        );

        Self {
            upstream,
            buffer: Vec::new(),
        }
    }

    fn proxy_request<W: Write>(
        &mut self,
        length: VarInt,
        cursor: Cursor<&[u8]>,
        next_packet_id: VarInt,
        response_writer: &mut W,
    ) {
        let mut write_cursor = cursor.clone();

        /* Read the data to proxy */
        let mut output_buffer = Vec::new();
        length
            .encode(&mut output_buffer)
            .expect("Failed to write length to output buffer");
        io::copy(&mut write_cursor, &mut output_buffer)
            .expect("Failed to write data to output buffer");
        next_packet_id
            .encode(&mut self.upstream)
            .expect("Failed to write extra data");

        self.upstream
            .write_all(&output_buffer)
            .expect("Failed to send message to remote server");

        // next_packet_id
        //     .encode(&mut self.upstream)
        //     .expect("Failed to write extra data");

        self.upstream
            .flush()
            .expect("Failed to flush data to upstream");

        info!("Sent packet");

        /* Read response from upstream */
        let response_length_raw = self
            .upstream
            .get_var_int()
            .expect("Failed to read length from upstream");
        let response_length = response_length_raw.0 as usize;

        info!("Got new response of length: {response_length}");

        // Resize buffer if needed
        if self.buffer.len() < response_length {
            self.buffer.resize(response_length, 0);
        }

        self.upstream
            .read_exact(&mut self.buffer[..response_length])
            .expect("Failed to read buffer");

        info!("Response bytes: {:?}", &self.buffer[..response_length]);

        response_length_raw
            .encode(response_writer)
            .expect("Failed to encode response length");

        let mut cursor = Cursor::new(&mut self.buffer);

        io::copy(&mut cursor, response_writer).expect("Failed to write response");
    }
}
