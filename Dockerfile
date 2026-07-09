FROM rust:1.96.0-slim-bookworm

RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    llvm \
    libclang-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY ./server .

RUN cargo build -p quorum-public --release

CMD ["./target/release/quorum-public"]