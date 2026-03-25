use super::*;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_env(vars: &[(&str, &str)], f: impl FnOnce() + std::panic::UnwindSafe) {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let all_keys = [
        "SERVER_URL",
        "AUTH_TOKEN",
        "LISTEN_ADDR",
        "LOG_LEVEL",
        "LOG_STYLE",
    ];
    unsafe {
        for key in &all_keys {
            std::env::remove_var(key);
        }
        for (key, value) in vars {
            std::env::set_var(key, value);
        }
    }
    let result = std::panic::catch_unwind(f);
    unsafe {
        for key in &all_keys {
            std::env::remove_var(key);
        }
    }
    if let Err(e) = result {
        std::panic::resume_unwind(e);
    }
}

#[test]
fn defaults_applied_when_env_unset() {
    with_env(&[], || {
        let config = Config::from_env();
        assert_eq!(config.server_url, "http://localhost:50051");
        assert!(config.auth_token.is_none());
        assert_eq!(config.listen_addr, "127.0.0.1:50052");
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_style, "auto");
    });
}

#[test]
fn env_vars_override_defaults() {
    with_env(
        &[
            ("SERVER_URL", "https://hyperfocus.example.com:50051"),
            ("AUTH_TOKEN", "secret-token-123"),
            ("LISTEN_ADDR", "0.0.0.0:9090"),
            ("LOG_LEVEL", "debug"),
            ("LOG_STYLE", "json"),
        ],
        || {
            let config = Config::from_env();
            assert_eq!(config.server_url, "https://hyperfocus.example.com:50051");
            assert_eq!(config.auth_token.as_deref(), Some("secret-token-123"));
            assert_eq!(config.listen_addr, "0.0.0.0:9090");
            assert_eq!(config.log_level, "debug");
            assert_eq!(config.log_style, "json");
        },
    );
}

#[test]
fn auth_token_is_optional() {
    with_env(&[], || {
        let config = Config::from_env();
        assert!(config.auth_token.is_none());
    });
}

#[test]
fn env_or_returns_default() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe { std::env::remove_var("__TEST_KEY_NONEXISTENT") };
    assert_eq!(env_or("__TEST_KEY_NONEXISTENT", "fallback"), "fallback");
}

#[test]
fn env_or_returns_env_value() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    unsafe {
        std::env::set_var("__TEST_KEY_EXISTS", "from_env");
    }
    assert_eq!(env_or("__TEST_KEY_EXISTS", "fallback"), "from_env");
    unsafe { std::env::remove_var("__TEST_KEY_EXISTS") };
}
