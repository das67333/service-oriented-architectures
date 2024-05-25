FROM rust:1.77-slim-buster AS builder
WORKDIR /build
RUN apt-get update && \
    apt-get install -y protobuf-compiler make g++ && \
    rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock build.rs posts.proto ./
# downloading dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release
COPY src ./src
RUN cargo build --release


FROM debian:bookworm-slim
COPY --from=builder /build/target/release/auth /app/auth
CMD ["/app/auth"]
