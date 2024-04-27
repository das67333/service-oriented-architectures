FROM rust:1.77-slim

WORKDIR /auth
COPY src ./src
COPY Cargo.toml Cargo.lock build.rs posts.proto ./
COPY protoc-26.1-linux-x86_64 /usr/local/
RUN apt update && \
    apt-get install make g++ -y && \
    rm -rf /var/lib/apt/lists/* && \
    cargo build --release && cp target/release/auth . && \
    cargo clean && \
    apt-get remove make g++ -y
CMD ["./auth"]
