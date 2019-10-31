#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export LIBRARY_NAME='transpiler'
export BINARY_NAME='zrustc'
export BUILD_MODE='debug'
export RUST_LOG="parser=${LOG_LEVEL},${LIBRARY_NAME}=${LOG_LEVEL},${BINARY_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

# *.zrs
export INPUT="${2}"

# *.rs
export OUTPUT="${3}"

cargo fmt --all
cargo test
cargo build --package "${LIBRARY_NAME}"

"./target/${BUILD_MODE}/${BINARY_NAME}" --meta --input "${INPUT}" --output "${OUTPUT}"

cargo run --bin circuit
