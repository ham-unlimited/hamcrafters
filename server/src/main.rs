use std::collections::HashMap;
use std::net::IpAddr;
use std::net::SocketAddr;

use eyre::Context;
use eyre::ContextCompat;
use log::info;

use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::select;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().wrap_err("Failed to read dotenv")?;
    color_eyre::install()?;
    pretty_env_logger::init();

    let host = "127.0.0.1:22211";
    let listener = TcpListener::bind(host)
        .await
        .wrap_err("Listening on {host}")?;

    let mut clients = ClientConnections::default();
    info!("Server listening on {host}");

    loop {
        select! {
            conn = listener.accept() => {
                let (stream, addr) = conn.wrap_err("Failed to receive incoming connection")?;

                info!("Receiving connection from {:?}", addr);

                let handle = tokio::task::spawn(handle_connection(stream));

                clients.add_client(&addr, handle);
            }
            finished = clients.finished_task() => {
                match finished {
                    Ok(_) => todo!(),
                    Err(_) => todo!(),
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct ClientConnections {
    clients: HashMap<ClientConnection, JoinHandle<eyre::Result<()>>>,
}

impl ClientConnections {
    async fn finished_task(&mut self) -> eyre::Result<Option<()>> {
        let conn = {
            let Some(conn) = self
                .clients
                .iter()
                .find(|(conn, handle)| handle.is_finished())
                .map(|(conn, _)| conn)
            else {
                return Ok(None);
            };

            (*conn).clone()
        };

        let handle = self
            .clients
            .remove(&conn)
            .wrap_err("Handle no longer exists?")?; // TODO: Wtf do we do if this is None? :sweat_smile:

        let res = handle.await.wrap_err("Failed to await finished handle?")?;

        Ok(Some(res.wrap_err("Error occurred when running task")?))
    }
}

impl ClientConnections {
    fn add_client(&mut self, addr: &SocketAddr, handle: JoinHandle<eyre::Result<()>>) {
        self.clients.insert(ClientConnection::from(addr), handle);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ClientConnection {
    src_ip: IpAddr,
    src_port: u16,
}

impl From<&SocketAddr> for ClientConnection {
    fn from(value: &SocketAddr) -> Self {
        Self {
            src_ip: value.ip(),
            src_port: value.port(),
        }
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

    let mut handler = ClientHandler::new(stream);
    handler
        .run()
        .await
        .wrap_err("Error occurred during running of program")?;

    Ok(())
}
