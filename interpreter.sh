#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export APPLICATION_NAME='interpreter'
export EXECUTABLE_NAME='jabi'
export BUILD_MODE='debug'

# *.jab
export INPUT="${2}"

export RUST_LOG="parser=${LOG_LEVEL},${APPLICATION_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --package "${APPLICATION_NAME}"
cargo test --package "${APPLICATION_NAME}"
cargo build --package "${APPLICATION_NAME}"

"./target/${BUILD_MODE}/${EXECUTABLE_NAME}" --meta --input "${INPUT}"
