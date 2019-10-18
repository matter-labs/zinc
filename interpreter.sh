#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export RUST_LOG="parser=${LOG_LEVEL},interpreter=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --package 'interpreter'
cargo build --package 'interpreter'

for file in ./examples/*.jab; do
    ./target/debug/jabi --meta --input "${file}"
done
