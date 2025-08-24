use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

/// Handles a single client connection
pub async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let peer = stream.peer_addr().ok();
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let response = format!("ST: {line}\n");
        writer.write_all(response.as_bytes()).await?;
        writer.flush().await?;
    }

    if let Some(addr) = peer {
        println!("[{addr}] disconnected");
    }
    Ok(())
}
