#!/bin/bash -Cerx

# 'jabc' | 'jabi' | 'jabserver'
export APPLICATION="${1}"

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${2}"

# *.jab
export INPUT="${3}"

# *.rs
export OUTPUT="${4}"

export RUST_LOG="compiler=${LOG_LEVEL},${APPLICATION}=${LOG_LEVEL}"
export RUST_BACKTRACE='0'

cargo fmt --all
cargo clippy
cargo test

cargo run --bin "${APPLICATION}" -- --input "${INPUT}"
# cargo run --bin "${APPLICATION}" -- --meta --input "${INPUT}" --output "${OUTPUT}"
