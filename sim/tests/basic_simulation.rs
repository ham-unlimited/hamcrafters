use client_handler::client_handler::ClientHandler;
use mc_coms::{
    SUPPORTED_MINECRAFT_PROTOCOL_VERSION, codec::var_int::VarInt,
    messages::serverbound::handshaking::handshake::Handshake, packet_writer::NetworkWriter,
};

#[test]
fn test_successful_handshake() {
    let mut sim = turmoil::Builder::new().build();

    // Server side - accept connection and run ClientHandler
    sim.host("server", || async {
        let listener = turmoil::net::TcpListener::bind("0.0.0.0:25565")
            .await
            .expect("Failed to bind server");

        let (stream, _addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let mut handler = ClientHandler::new(stream);

        // Run the handler - it should handle the handshake packet successfully
        // The handler will return Ok(()) when the connection is closed cleanly
        match handler.run().await {
            Ok(()) => println!("Server: Connection closed cleanly"),
            Err(e) => panic!("Server: Handler error: {:?}", e),
        }

        Ok(())
    });

    // Client side - connect and send handshake packet
    sim.client("client", async {
        let mut stream = turmoil::net::TcpStream::connect("server:25565")
            .await
            .expect("Failed to connect to server");

        // Create handshake packet using the struct
        let handshake = Handshake {
            protocol_version: VarInt(SUPPORTED_MINECRAFT_PROTOCOL_VERSION as i32),
            server_address: "localhost".to_string(),
            server_port: 25565,
            intent: VarInt(1), // Status
        };

        // Use NetworkWriter to serialize and send the packet
        let mut writer = NetworkWriter::new(&mut stream);
        writer
            .write_packet(handshake)
            .await
            .expect("Failed to write handshake packet");

        println!("Client: Sent handshake packet");

        // Close the connection
        drop(stream);

        Ok(())
    });

    sim.run().expect("Simulation failed");
}
