#!/usr/bin/env bash

set -Cex

# Logging level: 'error' | 'warn' | 'info' | 'debug' | 'trace'
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

# Target build: 'debug' | 'release'
case "${2}" in
    debug)
        export TARGET_DIRECTORY="debug"
        ;;
    release)
        export RELEASE_FLAG="--release"
        export TARGET_DIRECTORY="release"
        ;;
    *)
        export TARGET_DIRECTORY="debug"
        ;;
esac

# Zinc project name
export PROJECT_NAME="${3}"

# Zinc smart contract method name
export PROJECT_METHOD="${4}"

# Zinc integration tester: 'proof-check'?
case "${5}" in
    proof-check)
        export PROOF_CHECK="--proof-check"
        ;;
    *)
        export PROOF_CHECK=""
        ;;
esac

export ZARGO_PATH="./target/${TARGET_DIRECTORY}/zargo"
export ZINC_TESTER_NAME='zinc-tester'

#cargo fmt --all
#cargo clippy
cargo build ${CARGO_LOG_LEVEL} ${RELEASE_FLAG}
#cargo test
#cargo run ${CARGO_LOG_LEVEL} ${RELEASE_FLAG} --bin ${ZINC_TESTER_NAME} -- ${LOG_LEVEL} ${PROOF_CHECK}

#if [[ -n "${PROJECT_NAME}" ]]; then
#  export PROJECT_DIRECTORY="./zinc-examples/${PROJECT_NAME}/"
#  export MANIFEST_PATH="${PROJECT_DIRECTORY}/Zargo.toml"
#
#  "${ZARGO_PATH}" clean ${LOG_LEVEL} --manifest-path "${MANIFEST_PATH}"
#  "${ZARGO_PATH}" test ${LOG_LEVEL} --manifest-path "${MANIFEST_PATH}"
#
#  if [[ -n "${PROJECT_METHOD}" ]]; then
#    "${ZARGO_PATH}" proof-check ${LOG_LEVEL} ${RELEASE_FLAG} --manifest-path "${MANIFEST_PATH}" --method "${PROJECT_METHOD}"
#  else
#    "${ZARGO_PATH}" proof-check ${LOG_LEVEL} ${RELEASE_FLAG} --manifest-path "${MANIFEST_PATH}"
#  fi
#fi

cargo build ${CARGO_LOG_LEVEL} ${RELEASE_FLAG} --bin 'zargo'
cargo run ${CARGO_LOG_LEVEL} ${RELEASE_FLAG} --bin 'zandbox' -- ${LOG_LEVEL}
