#!/bin/bash -Cerx

# 'jabi' | 'jabc' | 'jabserver'
export APPLICATION="${1}"

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${2}"

# '**/*.jab'
export INPUT="${3}"

export RUST_LOG="compiler=${LOG_LEVEL},${APPLICATION}=${LOG_LEVEL}"
export RUST_BACKTRACE='0'

cargo run --bin "${APPLICATION}" "${INPUT}"
