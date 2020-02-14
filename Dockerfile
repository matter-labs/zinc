FROM rust:1.41 as builder-linux-musl
COPY . .

RUN apt update -y && apt install -y musl musl-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --verbose --release --target x86_64-unknown-linux-musl
