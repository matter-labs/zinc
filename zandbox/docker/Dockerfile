# syntax=docker/dockerfile:experimental
#
# From the root directory:
# DOCKER_BUILDKIT=1 docker build -t matterlabs/zandbox -f zandbox/docker/Dockerfile .
# docker push matterlabs/zandbox:0.2.1

FROM rust:1.48 as builder

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install sccache
WORKDIR /usr/src/zandbox
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/root/.cache/sccache \
    RUSTC_WRAPPER=/usr/local/cargo/bin/sccache \
    cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 3000
COPY --from=builder /usr/src/zandbox/target/release/zandbox /usr/bin
ENTRYPOINT ["sh", "-c", "zandbox --http-port 3000 -vv --postgresql $DATABASE_URL --network $ETH_NETWORK"]
