#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export LIBRARY_NAME='compiler'
export BINARY_NAME='zrustc'
export BUILD_MODE='debug'
export RUST_LOG="${LIBRARY_NAME}=${LOG_LEVEL},${BINARY_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

# *.zrs
export INPUT="${2}"

cargo fmt --package "${LIBRARY_NAME}"
cargo build --package "${LIBRARY_NAME}"

"./target/${BUILD_MODE}/${BINARY_NAME}" --meta --input "${INPUT}"
