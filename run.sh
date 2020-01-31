#!/usr/bin/env bash

set -ex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
case "${1}" in
    error)
        export LOG_LEVEL="error"
        ;;
    warn)
        export LOG_LEVEL="warn"
        ;;
    info)
        export LOG_LEVEL="info"
        ;;
    debug)
        export LOG_LEVEL="debug"
        ;;
    trace)
        export LOG_LEVEL="trace"
        ;;
    *)
        export LOG_LEVEL="info"
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

export ZARGO_CRATE_NAME='zargo'
export ZARGO_BINARY_NAME='zargo'

export COMPILER_CRATE_NAME='zinc-compiler'
export COMPILER_CRATE_NAME_LOG='zinc_compiler'
export COMPILER_BINARY_NAME='znc'

export VIRTUAL_MACHINE_CRATE_NAME='zinc-vm'
export VIRTUAL_MACHINE_CRATE_NAME_LOG='zinc_vm'
export VIRTUAL_MACHINE_BINARY_NAME='zinc'

export PROJECT_DIRECTORY='./test/'
export PROJECT_BUILD_DIRECTORY="${PROJECT_DIRECTORY}/build/"

export RUST_LOG="
${ZARGO_CRATE_NAME}=${LOG_LEVEL},\
${ZARGO_BINARY_NAME}=${LOG_LEVEL},\
${COMPILER_CRATE_NAME_LOG}=${LOG_LEVEL},\
${COMPILER_BINARY_NAME}=${LOG_LEVEL},\
${VIRTUAL_MACHINE_CRATE_NAME_LOG}=error,\
${VIRTUAL_MACHINE_BINARY_NAME}=error,\
"
export RUST_BACKTRACE=1

cargo fmt --all
cargo build ${RELEASE_MODE_FLAG} --package "${ZARGO_CRATE_NAME}"
cargo build ${RELEASE_MODE_FLAG} --package "${COMPILER_CRATE_NAME}"
cargo build ${RELEASE_MODE_FLAG} --package "${VIRTUAL_MACHINE_CRATE_NAME}"
cargo test

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}"
rm -fv "${PROJECT_DIRECTORY}/Zargo.toml"

"${ZARGO_PATH}" init "${PROJECT_DIRECTORY}"
"${ZARGO_PATH}" build \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"
"${ZARGO_PATH}" run \
    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
    --input "${PROJECT_BUILD_DIRECTORY}/witness.json" \
    --output "${PROJECT_BUILD_DIRECTORY}/public-data.json"
"${ZARGO_PATH}" setup \
    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt"
"${ZARGO_PATH}" prove \
    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
    --witness "${PROJECT_BUILD_DIRECTORY}/witness.json" \
    --pubdata "${PROJECT_BUILD_DIRECTORY}/public-data.json" > "${PROJECT_BUILD_DIRECTORY}/proof.txt"
"${ZARGO_PATH}" verify \
    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt" \
    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json" < "${PROJECT_BUILD_DIRECTORY}/proof.txt"
