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
        export TARGET_DIRECTORY="release"
        ;;
    *)
        export TARGET_DIRECTORY="debug"
        ;;
esac

export ZARGO_PATH="../../target/${TARGET_DIRECTORY}/zargo"

"${ZARGO_PATH}" clean ${LOG_LEVEL}
"${ZARGO_PATH}" build ${LOG_LEVEL}

echo '{ "witness_bool": true, "witness_integer": "42" }' > './build/witness.json'

"${ZARGO_PATH}" run ${LOG_LEVEL}
"${ZARGO_PATH}" setup ${LOG_LEVEL}
"${ZARGO_PATH}" prove | ${ZARGO_PATH} verify
