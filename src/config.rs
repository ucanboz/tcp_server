use std::env;

/// Holds configuration for the server (addr + concurrency).
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub max_concurrency: usize,
}

impl ServerConfig {
    /// Parse args or fallback to defaults
    pub fn from_args() -> Self {
        let mut args = env::args().skip(1);
        let addr = args.next().unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let max_concurrency = args
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);

        Self {addr, max_concurrency }
    }
}
