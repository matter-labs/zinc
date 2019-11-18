# Standard library

Most of the standard library functions will be embedded into the ZRust virtual
machine, which is described in the **Chapter 8**.

The standard library will provide computation-heavy algorithms like:
- hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
- signatures: `eddsa_verify`
- curve primitives: `ecc`
- into_bits / from_bits
