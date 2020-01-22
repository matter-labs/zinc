#!/bin/bash -Cex

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

export PROJECT_DIRECTORY='./examples/test/'

export RUST_LOG="
${ZARGO_CRATE_NAME}=${LOG_LEVEL},\
${ZARGO_BINARY_NAME}=${LOG_LEVEL},\
${COMPILER_CRATE_NAME_LOG}=${LOG_LEVEL},\
${COMPILER_BINARY_NAME}=${LOG_LEVEL},\
${VIRTUAL_MACHINE_CRATE_NAME_LOG}=${LOG_LEVEL},\
${VIRTUAL_MACHINE_BINARY_NAME}=${LOG_LEVEL},\
"
export RUST_BACKTRACE=1

cargo fmt --all

cargo build ${RELEASE_MODE_FLAG} --package "${ZARGO_CRATE_NAME}"

cargo build ${RELEASE_MODE_FLAG} --package "${COMPILER_CRATE_NAME}"
cargo test --package "${COMPILER_CRATE_NAME}"

cargo build ${RELEASE_MODE_FLAG} --package "${VIRTUAL_MACHINE_CRATE_NAME}"

"./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}" build \
    --manifest-path "${PROJECT_DIRECTORY}/Zargo.toml"
"./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}" exec \
    --circuit "${PROJECT_DIRECTORY}/build/default.znb" \
    --input "${PROJECT_DIRECTORY}/build/input.json" \
    --output "${PROJECT_DIRECTORY}/build/output.json"
"./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}" setup \
    --circuit "${PROJECT_DIRECTORY}/build/default.znb" \
    --params "${PROJECT_DIRECTORY}/build/params"
"./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}" prove \
    --circuit "${PROJECT_DIRECTORY}/build/default.znb" \
    --params "${PROJECT_DIRECTORY}/build/params" \
    --input "${PROJECT_DIRECTORY}/build/input.json" \
    --proof "${PROJECT_DIRECTORY}/build/proof"
"./target/${TARGET_DIRECTORY}/${ZARGO_BINARY_NAME}" verify \
    --circuit "${PROJECT_DIRECTORY}/build/default.znb" \
    --params "${PROJECT_DIRECTORY}/build/params" \
    --proof "${PROJECT_DIRECTORY}/build/proof" \
    --output "${PROJECT_DIRECTORY}/build/output.json"
