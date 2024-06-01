FROM rust:1.77-slim-buster AS builder
WORKDIR /build
RUN apt-get update && \
    apt-get install -y protobuf-compiler make g++ && \
    rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock build.rs *.proto ./
# downloading dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
COPY src ./src
RUN cargo build --release


FROM debian:bookworm-slim
WORKDIR /app
COPY init.sql .
COPY --from=builder /build/target/release/auth .
CMD ["./auth"]
