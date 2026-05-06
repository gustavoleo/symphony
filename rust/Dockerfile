FROM rust:1.87 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/symphony-rust /usr/local/bin/symphony-rust
ENV SYMPHONY_POLL_INTERVAL_SECS=30
ENV SYMPHONY_WORKSPACE_ROOT=/workspaces
ENV SYMPHONY_TRACKER_KIND=linear
CMD ["symphony-rust"]
