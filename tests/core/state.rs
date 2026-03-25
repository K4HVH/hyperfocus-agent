use super::*;

fn test_config() -> Config {
    Config {
        server_url: "http://localhost:50051".to_owned(),
        auth_token: None,
        listen_addr: "127.0.0.1:50052".to_owned(),
        log_level: "info".to_owned(),
        log_style: "plain".to_owned(),
    }
}

#[tokio::test]
async fn new_returns_arc() {
    let state = AppState::new(test_config());
    assert_eq!(Arc::strong_count(&state), 1);
}

#[tokio::test]
async fn config_returns_initial_values() {
    let state = AppState::new(test_config());
    let config = state.config();
    assert_eq!(config.listen_addr, "127.0.0.1:50052");
    assert_eq!(config.log_level, "info");
}

#[tokio::test]
async fn update_config_swaps_atomically() {
    let state = AppState::new(test_config());

    let new_config = Config {
        server_url: "https://prod.example.com:50051".to_owned(),
        auth_token: Some("token-abc".to_owned()),
        listen_addr: "0.0.0.0:9090".to_owned(),
        log_level: "debug".to_owned(),
        log_style: "json".to_owned(),
    };

    state.update_config(new_config);

    let config = state.config();
    assert_eq!(config.listen_addr, "0.0.0.0:9090");
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.log_style, "json");
    assert_eq!(config.server_url, "https://prod.example.com:50051");
}

#[tokio::test]
async fn started_at_is_recent() {
    let before = Instant::now();
    let state = AppState::new(test_config());
    let after = Instant::now();

    assert!(state.started_at() >= before);
    assert!(state.started_at() <= after);
}

#[tokio::test]
async fn uptime_secs_is_zero_initially() {
    let state = AppState::new(test_config());
    assert_eq!(state.uptime_secs(), 0);
}

#[tokio::test]
async fn health_registry_accessible() {
    let state = AppState::new(test_config());
    let _health = state.health();
}

#[tokio::test]
async fn config_is_cloneable_via_load() {
    let state = AppState::new(test_config());
    let config1 = state.config();
    let config2 = state.config();
    assert_eq!(config1.listen_addr, config2.listen_addr);
}
