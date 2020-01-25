#!/usr/bin/env bash

function cleanup {
    rm -rf Zargo.toml proof.txt build
}

set -ev

cleanup

zargo init --name test

zargo build
zargo run
zargo clean

zargo build
zargo setup
zargo prove > proof.txt
zargo verify < proof.txt
zargo clean

cleanup
