# hyperfocus-agent

Local agent for hyperfocus — a game macro SaaS tool. All code is proprietary.

## Architecture

The agent runs on the user's machine near the game. It connects to hyperfocus-server (gRPC client) to receive configuration and report status. It also serves a gRPC health endpoint that the server can query. Future subsystems include UDP video receiver, mouse driver, and macro execution.

Based on the MidnightServer template (`template` git remote for upstream fixes).

## Tech stack

- Rust 2024 edition
- gRPC: Tonic + Prost (both client and server)
- Config: environment variables via dotenvy
- State: ArcSwap for lock-free hot-reload of config
- Logging: tracing + tracing-subscriber (4 styles: plain, compact, pretty, json)
- Health: probe-based HealthRegistry for monitoring subsystems

## Development

```sh
cp .env.example .env
cargo run
```

## Build commands

```sh
cargo build
cargo clippy -- -D warnings
cargo fmt --check
cargo test
```

## Conventions

- Proto definitions are in `proto/hyperfocus/`, package name is `hyperfocus`
- Generated proto code is committed to `src/proto/generated/` (CI validates it)
- Proto files are copied from hyperfocus-server (shared contract)
- build.rs generates both client and server code (`build_client(true)` + `build_server(true)`)
- Tests use the `#[path]` attribute to include from `tests/` directory
- Config is env-var based — all fields have sensible defaults
- No database, no Docker — this is a native binary

## Related repos

- **hyperfocus-server** — Cloud gRPC server
- **hyperfocus-ui** — SolidJS web frontend (MidnightUI-based)
