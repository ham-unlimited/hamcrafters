use client_handler::client_handler::ClientHandler;
use eyre::Context;
use log::info;

use tokio::net::TcpListener;

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
