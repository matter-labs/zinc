# Introduction

ZRust: secure-by-design Rust-based language for constructing R1CS ZK-circuits and smart contracts.

The language is almost strictly a subset of simple Rust (with minor differences dictated by the specifics of R1CS circuits). It is easily learnable by golang, c++, solidity and js developers.

Its transpiler converts a program into an R1CS circuit using the [bellman](https://github.com/matter-labs/bellman) library. This allows generation of Zero Knowledge Proofs for any proof system supported by bellman (such as Groth16, PLONK, Marlin).
