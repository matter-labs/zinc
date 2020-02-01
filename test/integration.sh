#!/usr/bin/env bash

set -Cex

cd test
rm -fv './Zargo.toml'
zargo init --name test

zargo build
zargo run
zargo clean

zargo build
zargo setup
zargo prove > './build/proof.txt'
zargo verify < './build/proof.txt'
zargo clean
