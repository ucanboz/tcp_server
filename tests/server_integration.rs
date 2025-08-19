use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    task,
    time::{sleep, Duration},
};
use std::net::TcpListener;
use tcp_server::{config::ServerConfig, server::TcpServer};

fn get_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

/// Helper to spin up server on given addr and send/receive a test message
async fn run_echo_test(addr: &str) {
    let config = ServerConfig {
        addr: addr.to_string(),
        max_concurrency: 2,
    };

    task::spawn({
        let config = config.clone();
        async move {
            let mut server = TcpServer::new(config);
            let _ = server.run().await;
        }
    });

    // Give the server time to bind
    sleep(Duration::from_millis(200)).await;

    let stream = TcpStream::connect(addr).await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"addr_test\n").await.unwrap();
    writer.flush().await.unwrap();

    let response = lines.next_line().await.unwrap().unwrap();
    assert_eq!(response, "addr_test");
}

#[tokio::test]
async fn test_loopback_ipv4() {
    let port = get_free_port();
    let addr = format!("127.0.0.1:{port}");
    run_echo_test(&addr).await;
}

#[tokio::test]
async fn test_loopback_ipv6() {
    // Only run if IPv6 is supported on the machine
    if let Ok(listener) = TcpListener::bind("[::1]:0") {
        let port = listener.local_addr().unwrap().port();
        drop(listener);

        let addr = format!("[::1]:{port}");
        run_echo_test(&addr).await;
    }
}

#[tokio::test]
async fn test_wildcard_ipv4() {
    let port = get_free_port();
    let addr = format!("0.0.0.0:{port}");
    run_echo_test(&addr).await;
}


