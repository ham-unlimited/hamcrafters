mod encryption;

use eyre::Context;
use log::info;

use tokio::net::TcpListener;
use tokio::net::TcpStream;

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
        // TODO: Spin up a new thread for each client.
        let (stream, addr) = listener
            .accept()
            .await
            .wrap_err("Failed to receive incoming connection")?;

        info!("Receiving connection from {:?}", addr);

        handle_connection(stream)
            .await
            .wrap_err("Failed to handle connection")?;
    }
}

#[cfg(feature = "proxy")]
async fn handle_connection(stream: TcpStream) -> eyre::Result<()> {
    use proxy::ProxyHandler;

    info!("Setting up proxy...");
    let mut handler = ProxyHandler::new(stream)
        .await
        .wrap_err("Failed to setup proxy")?;
    handler
        .run()
        .await
        .wrap_err("Error occurred during running of proxy")?;

    Ok(())
}

#[cfg(not(feature = "proxy"))]
async fn handle_connection(stream: TcpStream) -> eyre::Result<()> {
    use client_handler::client_handler::ClientHandler;

    use crate::encryption::encryption::McEncryptionKeys;

    let keys = McEncryptionKeys::new();

    let mut handler = ClientHandler::new(stream, keys.der_encode_pub_key(), keys.priv_key);

    handler
        .run()
        .await
        .wrap_err("Error occurred during running of program")?;

    Ok(())
}
