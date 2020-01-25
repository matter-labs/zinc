# Standard library

The standard library is unstable. Function signatures and behavior are going to
be changed in future releases.

Hash functions described here are special, as they accept a byte array of
arbitrary size. Since there are only fixed-size arrays in Zinc now, it would
be challenging to create a function for array of every possible size. It is
not possible to write such a function yourself using the language type
system, but `std` makes an exception to simpilify development.

## `std::sha256`

Computes the `sha256` hash of a given byte array.

Arguments:
- preimage byte array `[u8; N]`

Returns: 256-bit hash `[u8; 32]`

## `std::pedersen`

Maps a byte array to a point on an elliptic curve.

To understand what is under the hood, see [this article](https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/pedersen-hash/pedersen.html).

Arguments:
- preimage byte array `[u8; N]`

Returns: point coordinates `(field, field)`
