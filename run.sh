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
        ;;
    debug)
        export LOG_LEVEL="-vv"
        export RUST_BACKTRACE=1
        ;;
    trace)
        export LOG_LEVEL="-vvv"
        export RUST_BACKTRACE="full"
        ;;
    *)
        export LOG_LEVEL="-v"
        ;;
esac

# 'debug' | 'release'
case "${2}" in
    release)
        export RELEASE_MODE_FLAG="--release"
        export TARGET_DIRECTORY="release"
        ;;
    *)
        export TARGET_DIRECTORY="debug"
        ;;
esac

export PROJECT_DIRECTORY='./zinc-tests/casual/'
export PROJECT_BUILD_DIRECTORY="${PROJECT_DIRECTORY}/build/"

cargo fmt --all
cargo clippy
cargo build ${RELEASE_MODE_FLAG}
cargo test

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/zargo"

rm -fv "${PROJECT_DIRECTORY}/Zargo.toml"
"${ZARGO_PATH}" init ${LOG_LEVEL} "${PROJECT_DIRECTORY}"
"${ZARGO_PATH}" clean ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"
"${ZARGO_PATH}" build ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml" \
    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
    --witness "${PROJECT_BUILD_DIRECTORY}/witness.json" \
    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json"
#"${ZARGO_PATH}" run ${LOG_LEVEL} \
#    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
#    --witness "${PROJECT_BUILD_DIRECTORY}/witness.json" \
#    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json"
#"${ZARGO_PATH}" setup ${LOG_LEVEL} \
#    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
#    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
#    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt"
#"${ZARGO_PATH}" prove ${LOG_LEVEL} \
#    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
#    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
#    --witness "${PROJECT_BUILD_DIRECTORY}/witness.json" \
#    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json" > "${PROJECT_BUILD_DIRECTORY}/proof.txt"
#"${ZARGO_PATH}" verify ${LOG_LEVEL} \
#    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt" \
#    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json" < "${PROJECT_BUILD_DIRECTORY}/proof.txt"
