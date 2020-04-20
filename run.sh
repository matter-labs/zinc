#!/usr/bin/env bash

set -ex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
case "${1}" in
    error)
        export LOG_LEVEL=""
        ;;
    warn)
        export LOG_LEVEL=""
        ;;
    info)
        export LOG_LEVEL="-v"
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    debug)
        export LOG_LEVEL="-vv"
        export RUST_BACKTRACE=1
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    trace)
        export LOG_LEVEL="-vvv"
        export RUST_BACKTRACE="full"
        export CARGO_LOG_LEVEL="--verbose"
        ;;
    *)
        export LOG_LEVEL=""
        ;;
esac

# 'debug' | 'release'
case "${2}" in
    debug)
        export TARGET_DIRECTORY="debug"
        ;;
    release)
        export RELEASE_MODE_FLAG="--release"
        export TARGET_DIRECTORY="release"
        ;;
    *)
        export TARGET_DIRECTORY="debug"
        ;;
esac

export CIRCUIT_DIRECTORY='./zinc-examples/casual/'
export CIRCUIT_BUILD_DIRECTORY="${CIRCUIT_DIRECTORY}/build/"
export CIRCUIT_DATA_DIRECTORY="${CIRCUIT_DIRECTORY}/data/"

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/zargo"

cargo fmt --all
cargo clippy
cargo build ${CARGO_LOG_LEVEL} ${RELEASE_MODE_FLAG}
cargo test
cargo run ${CARGO_LOG_LEVEL} ${RELEASE_MODE_FLAG} --bin 'zinc-tester' -- ${LOG_LEVEL}

"${ZARGO_PATH}" clean ${LOG_LEVEL} \
    --manifest-path "${CIRCUIT_DIRECTORY}/Zargo.toml"

"${ZARGO_PATH}" proof-check ${LOG_LEVEL} \
    --manifest-path "${CIRCUIT_DIRECTORY}/Zargo.toml" \
    --circuit "${CIRCUIT_BUILD_DIRECTORY}/default.znb" \
    --witness "${CIRCUIT_DATA_DIRECTORY}/witness.json" \
    --public-data "${CIRCUIT_DATA_DIRECTORY}/public-data.json" \
    --proving-key "${CIRCUIT_DATA_DIRECTORY}/proving-key" \
    --verifying-key "${CIRCUIT_DATA_DIRECTORY}/verifying-key.txt"
