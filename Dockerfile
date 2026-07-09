FROM rust:1.80-slim-bookworm

RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    llvm \
    libclang-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release -p quorum-public

CMD ["./server/target/release/quorum-public"]