#!/usr/bin/env bash

set -Cex

# Logging level: 'error' | 'warn' | 'info' | 'debug' | 'trace'
case "${1}" in
    error)
        export LOG_LEVEL=""
        ;;
    warn)
        export LOG_LEVEL=""
        ;;
    info)
        export LOG_LEVEL="-v"
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    debug)
        export LOG_LEVEL="-vv"
        export RUST_BACKTRACE=1
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    trace)
        export LOG_LEVEL="-vvv"
        export RUST_BACKTRACE="full"
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    *)
        export LOG_LEVEL=""
        ;;
esac

# Target build: 'debug' | 'release'
case "${2}" in
    debug)
        export TARGET_DIRECTORY="debug"
        ;;
    release)
        export RELEASE_FLAG="--release"
        export TARGET_DIRECTORY="release"
        ;;
    *)
        export TARGET_DIRECTORY="debug"
        ;;
esac

cargo fmt --all
cargo clippy
cargo test
cargo build ${CARGO_LOG_LEVEL} ${RELEASE_FLAG}

source './zandbox/.env'
cargo run ${CARGO_LOG_LEVEL} ${RELEASE_FLAG} --bin 'zandbox' -- ${LOG_LEVEL} \
  --network "${NETWORK}" \
  --postgresql "${DATABASE_URL}"
