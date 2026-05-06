# Symphony Rust (Preview)

This is a minimal Rust preview implementation of Symphony's long-running service loop.

## Run locally

```bash
cargo run
```

## Build release binary

```bash
cargo build --release
```

## Deploy with Docker Compose

From the `rust/` directory:

```bash
docker compose up -d --build
```

Check logs:

```bash
docker compose logs -f symphony-rust
```

Stop:

```bash
docker compose down
```

## Environment variables

- `SYMPHONY_POLL_INTERVAL_SECS` (default: `30`)
- `SYMPHONY_WORKSPACE_ROOT` (default: `./workspaces`)
- `SYMPHONY_TRACKER_KIND` (default: `linear`)
