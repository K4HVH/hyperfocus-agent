# hyperfocus-agent

The local agent for [hyperfocus](https://hyperfocus.k4tech.net) — a 2PC game macro platform. Runs on the second PC, processes video from the gaming PC, and controls the mouse via [MAKCU](https://github.com/k4hvh/makcu) hardware.

Hyperfocus is the successor to [FOCUS](https://github.com/K4HVH/focus-cpp). Where FOCUS ran everything on the gaming PC (making it vulnerable to detection), hyperfocus uses a 2PC architecture: PC1 streams its screen via an OBS plugin, PC2 (this agent) does all processing and sends mouse commands back through MAKCU hardware.

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

## How it works

```
[Mouse] --USB--> [MAKCU] --USB--> [PC1 (game)]
                    ^                  |
                    | USB serial    OBS plugin (UDP video x2)
                    |                  |
                 [PC2 (agent)] <-------+
                    |
                    | gRPC
                    v
              [Cloud server] <---> [Web UI]
```

- **PC1** runs the game and an OBS plugin that sends two UDP video streams to PC2
- **PC2** runs this agent, which processes the streams (detectors, AI) and sends mouse commands to MAKCU
- **MAKCU** hardware sits between the mouse and PC1, intercepting/injecting mouse traffic via USB passthrough
- The agent connects to the cloud server via gRPC for config, auth, and status reporting
- The agent has no UI — all user control is through the web interface

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

## Releases

CI automatically builds release binaries for all platforms when the version in `Cargo.toml` is bumped on main:

- Linux (x86_64, aarch64)
- Windows (x86_64, aarch64)
- macOS (x86_64, aarch64)

## Template

Based on [MidnightServer](https://github.com/k4hvh/midnightserver). The `template` remote is kept for pulling upstream fixes:

```sh
git fetch template
git merge template/main --allow-unrelated-histories
```
