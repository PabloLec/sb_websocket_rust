FROM rust:1-slim-buster AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/sb_websocket_rust /usr/local/bin/

CMD ["sb_websocket_rust"]