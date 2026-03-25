# hyperfocus-agent

Local agent for [hyperfocus](https://hyperfocus.k4tech.net) — a game macro tool. Runs near the game, receives configuration from the server, and executes macros.

## Setup

### System dependencies

```sh
# Fedora
sudo dnf install protobuf-compiler protobuf-devel

# Ubuntu / Debian
sudo apt install protobuf-compiler libprotobuf-dev
```

[Rust](https://rustup.rs/) (edition 2024).

### Run

```sh
cp .env.example .env
cargo run
```

Starts the agent and exposes a health endpoint on `127.0.0.1:50052`.

### Test

```sh
cargo test
```

## Configuration

All via environment variables (see [.env.example](.env.example)):

| Variable | Default | Description |
|---|---|---|
| `SERVER_URL` | `http://localhost:50051` | hyperfocus-server gRPC endpoint |
| `AUTH_TOKEN` | *(optional)* | Authentication token for server |
| `LISTEN_ADDR` | `127.0.0.1:50052` | Agent health endpoint bind address |
| `LOG_LEVEL` | `info` | Tracing filter directive |
| `LOG_STYLE` | `auto` | `plain`, `compact`, `pretty`, `json`, or `auto` |

## Project layout

```
proto/hyperfocus/        Protobuf definitions (shared with server)
src/
  main.rs                Agent entrypoint
  core/
    config.rs            Env-based config
    error.rs             AppError -> gRPC Status
    health.rs            Probe-based HealthRegistry
    logging.rs           Tracing setup (4 styles)
    state.rs             AppState (config, health, uptime)
  grpc/
    health.rs            Health service RPCs
  proto/                 Generated protobuf code
tests/                   Unit tests
```

## Template

Based on [MidnightServer](https://github.com/k4hvh/midnightserver). The `template` remote is kept for pulling upstream fixes:

```sh
git fetch template
git merge template/main --allow-unrelated-histories
```
