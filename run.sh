#!/bin/bash -Cex

# 'jabc' | 'jabi' | 'jabserver'
export APPLICATION="${1}"

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${2}"

# *.jab
export INPUT="${3}"

# *.rs
export OUTPUT="${4}"

export RUST_LOG="compiler=${LOG_LEVEL},${APPLICATION}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --all
cargo clippy
cargo test
cargo build --all

if [[ "${APPLICATION}" == 'jabc' ]]; then
    ./target/debug/jabc --meta --input "${INPUT}" --output "${OUTPUT}"
fi

if [[ "${APPLICATION}" == 'jabi' ]]; then
    for file in ./examples/*.jab; do
        ./target/debug/jabi --input "${file}"
    done
fi
