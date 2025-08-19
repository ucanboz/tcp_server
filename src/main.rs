use tcp_server::config::ServerConfig;
use tcp_server::server::TcpServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ServerConfig::from_args();
    let mut server = TcpServer::new(config);
    server.run().await
}
