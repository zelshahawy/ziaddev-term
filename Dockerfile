FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/ziad-curl /app/ziad-curl

ENV PORT=8080

EXPOSE 8080

CMD ["/app/ziad-curl"]
