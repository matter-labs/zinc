#!/usr/bin/env bash

set -Cex

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

export PROJECT_NAME="${3}"
export PROJECT_ENTRY="${4}"

export PROJECT_DIRECTORY="./zinc-examples/${PROJECT_NAME}/"
export PROJECT_BUILD_DIRECTORY="${PROJECT_DIRECTORY}/build/"
export PROJECT_BUILD_TEST_DIRECTORY="${PROJECT_BUILD_DIRECTORY}/test/"
export PROJECT_DATA_DIRECTORY="${PROJECT_DIRECTORY}/data/"
export PROJECT_SOURCE_DIRECTORY="${PROJECT_DIRECTORY}/src/"

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/zargo"

cargo fmt --all
cargo clippy
cargo build ${CARGO_LOG_LEVEL} ${RELEASE_MODE_FLAG}
cargo test
cargo run ${CARGO_LOG_LEVEL} ${RELEASE_MODE_FLAG} --bin 'zinc-tester' -- ${LOG_LEVEL} #--proof-check

"${ZARGO_PATH}" clean ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"

"${ZARGO_PATH}" proof-check ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml" \
    --binary "${PROJECT_BUILD_DIRECTORY}/${PROJECT_ENTRY}.znb" \
    --witness "${PROJECT_DATA_DIRECTORY}/${PROJECT_ENTRY}_witness.json" \
    --public-data "${PROJECT_DATA_DIRECTORY}/${PROJECT_ENTRY}_public_data.json" \
    --proving-key "${PROJECT_DATA_DIRECTORY}/proving_key" \
    --verifying-key "${PROJECT_DATA_DIRECTORY}/verifying_key.txt"

"${ZARGO_PATH}" test ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml" \
    --binary "${PROJECT_BUILD_TEST_DIRECTORY}"

cargo run ${CARGO_LOG_LEVEL} ${RELEASE_MODE_FLAG} --bin 'zinc-server' -- ${LOG_LEVEL} --port 80
