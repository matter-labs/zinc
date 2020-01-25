#!/usr/bin/env bash

function cleanup {
    rm -rf Zargo.toml proof.txt build
}

set -ev

cleanup

zargo init --name test 2> /dev/null

zargo build 2> /dev/null
zargo run > /dev/null
zargo clean 2> /dev/null

zargo build 2> /dev/null
zargo setup > /dev/null
zargo prove > proof.txt
zargo verify < proof.txt
zargo clean 2> /dev/null

cleanup
