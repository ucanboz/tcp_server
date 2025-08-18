mod config;
mod connection;
mod server;

use crate::config::ServerConfig;
use crate::server::TcpServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::from_args();
    let mut server = TcpServer::new(config);
    server.run().await
}
