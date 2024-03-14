FROM rust:1.76-slim

WORKDIR /auth
RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq-dev && \
    apt-get clean
COPY . .
RUN cargo build && \
    mv target/debug/auth . && \
    cargo clean
CMD ["./auth"]