#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

# *.jab
export INPUT="${2}"

# *.rs
export OUTPUT="${3}"

export RUST_LOG="parser=${LOG_LEVEL},transpiler=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --package 'transpiler'
cargo build --package 'transpiler'

./target/debug/jabc --meta --input "${INPUT}" --output "${OUTPUT}"

cargo run --bin circuit
