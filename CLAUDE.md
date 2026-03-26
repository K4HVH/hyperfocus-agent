# hyperfocus-agent

The "brawn" of hyperfocus — a 2PC game macro SaaS tool (successor to [FOCUS](https://github.com/K4HVH/focus-cpp)). All code is proprietary.

## What is hyperfocus?

Hyperfocus is a paid, subscription-based game macro platform. Unlike FOCUS (which was 1PC), hyperfocus uses a 2PC architecture to minimize detection. PC1 (the gaming PC) runs only an OBS plugin that captures the screen and sends two UDP video streams to PC2. PC2 runs this agent, which processes the video, runs detectors, and controls the mouse on PC1 via MAKCU hardware.

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

## What this agent does

The agent runs on PC2 (the processing PC). It has **no UI** — all user interaction is through the web UI via the server. It is:
- A **gRPC client** — connects to hyperfocus-server for config, auth, updates
- A **gRPC server** — exposes health endpoint so the server can monitor it
- A **video processor** — receives two UDP streams from the OBS plugin on PC1:
  - Low-latency ROI stream (for AI aimbot)
  - Fullscreen stream (for game detectors: weapon detection, operator detection, etc.)
- A **mouse controller** — sends commands to [MAKCU](https://github.com/k4hvh/makcu) hardware via USB serial (MAKCU sits between mouse and PC1, intercepting/injecting mouse traffic)
- A **macro engine** — recoil control, AI aimbot, game-specific logic

The agent runs in a **0-trust environment** (customer hardware) and must be protected against reverse engineering and tampering.

## Tech stack

- Rust 2024 edition
- gRPC: Tonic + Prost (both client and server)
- Config: environment variables via dotenvy
- State: ArcSwap for lock-free hot-reload of config
- Logging: tracing + tracing-subscriber (4 styles: plain, compact, pretty, json)
- Health: probe-based HealthRegistry for monitoring subsystems

## Planned subsystems (not yet implemented)

- UDP video receiver (2 streams)
- MAKCU mouse driver (USB serial)
- Game detectors (weapon, operator, etc.)
- Recoil engine
- AI aimbot
- Module registry (see below)
- Auto-updater

## Data module system

The agent uses a **data-driven architecture**. All detection/macro code is compiled into the agent binary, but the behavior is parameterized by **data modules** fetched from the server and held in memory (never written to disk).

A user's config references the modules it needs. When the agent loads a config, it pulls the required modules from the server. Examples of modules:

- **Character detection** — reference hashes + screen regions + icon dimensions (different per game)
- **Recoil tables** — per-weapon X/Y movement arrays and timing data
- **Attachment modifiers** — multipliers/curves that modify recoil patterns (different slot counts and semantics per game)
- **ML model weights** — for AI aimbot (the inference runner is compiled, models are data)
- **Screen region definitions** — where the ammo counter, health bar, ability icons, etc. are

The same compiled detector type (e.g. hash-based icon matcher) works across games — only the reference data changes. This means:

- New game support = new data modules on the server, no agent release needed
- Game balance patches = updated recoil tables pushed from server
- The "store" sells game packs (collections of data modules), not code
- Modules stay in memory only — nothing written to disk on the customer's machine
- No dynamic code loading — no plugin API to reverse-engineer

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

## CI/CD

CI runs on every push/PR (check, clippy, test). On push to main, if the version in `Cargo.toml` doesn't have a matching GitHub release, it builds release binaries for 6 targets (linux/windows/macos x x86_64/aarch64) and creates a GitHub release.

## Conventions

- Proto definitions are in `proto/hyperfocus/`, package name is `hyperfocus`
- Generated proto code is committed to `src/proto/generated/` (CI validates it)
- Proto files are copied from hyperfocus-server (shared contract)
- build.rs generates both client and server code (`build_client(true)` + `build_server(true)`)
- Tests use the `#[path]` attribute to include from `tests/` directory
- Config is env-var based — all fields have sensible defaults
- No database, no Docker — this is a native binary distributed to end users

## Related repos

- **hyperfocus-server** — Cloud gRPC server (the "brains")
- **hyperfocus-ui** — SolidJS web frontend (MidnightUI-based)
- **hyperfocus-obs-plugin** — OBS plugin on PC1 (already built)
