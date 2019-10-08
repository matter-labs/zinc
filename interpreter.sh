#!/bin/bash -Ce

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export RUST_LOG="parser=${LOG_LEVEL},interpreter=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --all
cargo clippy
cargo test
cargo build --all

for file in ./examples/*.jab; do
    ./target/debug/jabi --meta --input "${file}"
done
