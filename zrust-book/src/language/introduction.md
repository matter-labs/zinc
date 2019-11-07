# Introduction

ZRust is a secure-by-design language for constructing R1CS ZKP-circuits and
smart contracts.

Most of the modern languages, both general purpose and domain specific, do not
usually provide the satisfying degree of security in order to develop reliable
software. Security and safety aspects are crucial for developing smart
contracts, since they deal with digital assets of real people. On the contrary,
some languages are secure and safe enough, but they overwhelm developers with
overcomplicated syntax, thus decreasing their popularity by increasing the
learning curve. ZRust is designed to break this stereotype and prove that a
smart contract language can be reliable and simple at the same time.

The decision to borrow the Rust syntax and semantics has been made. So,
ZRust is a subset of Rust with minor differences dictated by the subtleties
of R1CS circuits. It is easily learnable by any developer familiar with
Rust, Golang, C++ or other C-like languages. Also, experience with Solidity
will help in understanding some smart contract specifics.

The language is under heavy development, so a great deal of the book contents
will be eventually rewritten. However, the basic language philosophy principles,
such as security and simplicity, will never be questioned.
