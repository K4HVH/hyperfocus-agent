use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use std::net::SocketAddr;

use tonic::transport::Server;

mod core;
mod grpc;
mod proto;

use core::state::AppState;
use proto::health_service_server::HealthServiceServer;

pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/descriptors.bin"));

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    let config = core::config::Config::from_env();
    core::logging::init(&config);

    let addr: SocketAddr = config.listen_addr.parse()?;
    let server_url = config.server_url.clone();

    let state = AppState::new(config);

    state
        .health()
        .register(
            "agent",
            Duration::from_secs(60),
            Some(env!("CARGO_PKG_VERSION").to_owned()),
            Box::new(|| Box::pin(async { Ok(()) })),
        )
        .await;

    let health_service = grpc::health::HealthServiceImpl::new(Arc::clone(&state));

    tracing::info!("hyperfocus-agent listening on {addr}");
    tracing::info!("configured server: {server_url}");

    Server::builder()
        .add_service(HealthServiceServer::new(health_service))
        .serve_with_shutdown(addr, shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("received Ctrl+C, shutting down"),
        _ = terminate => tracing::info!("received SIGTERM, shutting down"),
    }
}
