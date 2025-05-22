FROM rust:1.87.0-bookworm AS builder

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y musl-tools build-essential mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./agent/ ./agent/
COPY ./server/ ./server/
COPY ./shared/ ./shared/

RUN cargo build --release --target x86_64-pc-windows-gnu -p agent

RUN cargo build --release --target x86_64-unknown-linux-musl -p server


FROM alpine:3.21 AS runner

WORKDIR /app
COPY --from=builder /app/target/x86_64-pc-windows-gnu/release/agent.exe /app/agent.exe
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app/server
RUN chmod +x /app/server

EXPOSE 80

CMD ["/app/server"]