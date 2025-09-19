# ─── Builder (all on Debian Bookworm) ────────────────────────────────────────────
FROM rust:1.87.0-bookworm AS builder

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    pkg-config \
    libpq-dev \
    libssl-dev \
    mingw-w64-tools \
    gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 \
    && rm -rf /var/lib/apt/lists/*

# Enable the Windows target for your agent
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /app

# Copy manifests first (to cache deps)
COPY Cargo.toml Cargo.lock ./
COPY shared/ shared/

# Copy in agent and server crates
COPY agent/ agent/
COPY server/ server/

# 1) Build the Windows agent
RUN cargo build --release --target x86_64-pc-windows-gnu -p agent

# 2) Build the Linux server (default x86_64-unknown-linux-gnu)
RUN cargo build --release -p server

# 3) Install diesel_cli (Postgres) for the GNU target
RUN cargo install \
    diesel_cli \
    --no-default-features \
    --features postgres \
    --root /app/diesel-install


# ─── Runner (Debian Bookworm-slim) ───────────────────────────────────────────────
FROM debian:bookworm-slim AS runner

RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/server ./server
COPY --from=builder /app/diesel-install/bin/diesel /usr/local/bin/diesel

COPY --from=builder /app/target/x86_64-pc-windows-gnu/release/agent.exe ./agent.exe

RUN chmod +x server /usr/local/bin/diesel

EXPOSE 80

CMD ["./server"]
