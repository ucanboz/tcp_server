use crate::{config::ServerConfig, connection::handle_connection, gpio::gpio_blink_task};
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    signal,
    sync::{watch, Semaphore},
    task::JoinSet,
};

pub struct TcpServer {
    config: ServerConfig,
}

impl TcpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.config.addr).await?;
        println!(
            "Listening on {} with max {} concurrent connections…",
            self.config.addr, self.config.max_concurrency
        );

        // spawn background GPIO blinker
        tokio::spawn(async {
            gpio_blink_task().await;
        });

        // Wrap semaphore in Arc so it can be cloned and shared with tasks
        let limiter = Arc::new(Semaphore::new(self.config.max_concurrency));

        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);
        tokio::spawn(async move {
            let _ = signal::ctrl_c().await;
            let _ = shutdown_tx.send(true);
        });

        let mut tasks = JoinSet::new();

        loop {
            tokio::select! {
                _ = shutdown_rx.changed() => {
                    println!("Shutdown requested. Stopping accept loop…");
                    break;
                }
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((stream, peer)) => {
                            let limiter = limiter.clone(); // Arc<Semaphore>
                            tasks.spawn(async move {
                                let permit = limiter.acquire_owned().await.unwrap();
                                if let Err(e) = handle_connection(stream).await {
                                    eprintln!("[{peer}] error: {e}");
                                }
                                drop(permit);
                            });
                        }
                        Err(e) => {
                            eprintln!("Accept error: {e}");
                        }
                    }
                }
            }
        }

        println!("Waiting for active connections to finish…");
        while let Some(res) = tasks.join_next().await {
            if let Err(e) = res {
                eprintln!("A connection task panicked: {e}");
            }
        }
        println!("Server stopped cleanly.");
        Ok(())
    }
}
