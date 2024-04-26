FROM rust:1.77-slim

WORKDIR /auth
COPY src ./src
COPY Cargo.toml Cargo.lock build.rs posts.proto ./
COPY protoc-26.1-linux-x86_64 /usr/local/
RUN cargo build --release && cp target/release/auth . && cargo clean
CMD ["./auth"]
