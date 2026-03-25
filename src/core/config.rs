#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config {
    pub server_url: String,
    pub auth_token: Option<String>,
    pub listen_addr: String,
    pub log_level: String,
    pub log_style: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server_url: env_or("SERVER_URL", "http://localhost:50051"),
            auth_token: std::env::var("AUTH_TOKEN").ok(),
            listen_addr: env_or("LISTEN_ADDR", "127.0.0.1:50052"),
            log_level: env_or("LOG_LEVEL", "info"),
            log_style: env_or("LOG_STYLE", "auto"),
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

#[cfg(test)]
#[path = "../../tests/core/config.rs"]
mod tests;
