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

export ZARGO_CRATE_NAME='zargo'
export ZARGO_BINARY_NAME='zargo'

export COMPILER_CRATE_NAME='zinc-compiler'
export COMPILER_CRATE_NAME_LOG='zinc_compiler'
export COMPILER_BINARY_NAME='znc'

export VIRTUAL_MACHINE_CRATE_NAME='zinc-vm'
export VIRTUAL_MACHINE_CRATE_NAME_LOG='zinc_vm'
export VIRTUAL_MACHINE_BINARY_NAME='zinc'

export PROJECT_DIRECTORY='./debug/'
export PROJECT_BUILD_DIRECTORY="${PROJECT_DIRECTORY}/build/"

cargo fmt --all
cargo build ${RELEASE_MODE_FLAG} --package "${ZARGO_CRATE_NAME}"
cargo build ${RELEASE_MODE_FLAG} --package "${COMPILER_CRATE_NAME}"
cargo build ${RELEASE_MODE_FLAG} --package "${VIRTUAL_MACHINE_CRATE_NAME}"
#cargo test

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}"
rm -fv "${PROJECT_DIRECTORY}/Zargo.toml"

"${ZARGO_PATH}" init ${LOG_LEVEL} "${PROJECT_DIRECTORY}"
"${ZARGO_PATH}" clean ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"
"${ZARGO_PATH}" build ${LOG_LEVEL} \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"
"${ZARGO_PATH}" run ${LOG_LEVEL} \
    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
    --input "${PROJECT_BUILD_DIRECTORY}/witness.json" \
    --output "${PROJECT_BUILD_DIRECTORY}/public-data.json"
#"${ZARGO_PATH}" setup ${LOG_LEVEL} \
#    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
#    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
#    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt"
#"${ZARGO_PATH}" prove ${LOG_LEVEL} \
#    --circuit "${PROJECT_BUILD_DIRECTORY}/default.znb" \
#    --proving-key "${PROJECT_BUILD_DIRECTORY}/proving-key" \
#    --witness "${PROJECT_BUILD_DIRECTORY}/witness.json" \
#    --pubdata "${PROJECT_BUILD_DIRECTORY}/public-data.json" > "${PROJECT_BUILD_DIRECTORY}/proof.txt"
#"${ZARGO_PATH}" verify ${LOG_LEVEL} \
#    --verifying-key "${PROJECT_BUILD_DIRECTORY}/verifying-key.txt" \
#    --public-data "${PROJECT_BUILD_DIRECTORY}/public-data.json" < "${PROJECT_BUILD_DIRECTORY}/proof.txt"
