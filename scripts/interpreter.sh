#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export APPLICATION_NAME='interpreter'
export EXECUTABLE_NAME='zrusti'
export BUILD_MODE='debug'
export RUST_LOG="parser=${LOG_LEVEL},${APPLICATION_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

# *.zrs
export INPUT="${2}"

cargo fmt --all
cargo test
cargo build --package "${APPLICATION_NAME}"

"./target/${BUILD_MODE}/${EXECUTABLE_NAME}" --meta --input "${INPUT}"
