# Standard library

The standard library is unstable. Function signatures and behavior are going to
be changed in future releases.

Most of the functions described here are special, as they accept arrays of
arbitrary size. Since there are only fixed-size arrays in Zinc now, it would
be challenging to create a function for arrays of every possible size. It is
not possible to write such a function yourself using the language type
system, but `std` makes an exception to simplify development for now.

## Definitions

- `{scalar}` - a scalar type, which can be `bool`, `u{N}`, `i{N}`, `field`
- `u{N}` - an unsigned integer of bitlength `N`
- `i{N}` - a signed integer of bitlength `N`
- `field` - a field element of bitlength `254`

## `crypto` module

### `std::crypto::sha256`

Computes the `sha256` hash of a given bit array.

Arguments:
- preimage bit array `[bool; 8*N]` (the size must be multiple of 8)

Returns: 256-bit hash `[bool; 256]`

### `std::crypto::pedersen`

Maps a bit array to a point on an elliptic curve.

To understand what is under the hood, see [this article](https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/pedersen-hash/pedersen.html).

Arguments:
- preimage bit array `[bool; N]`

Returns: elliptic curve point coordinates `(field, field)`

## `convert` module

### `std::convert::to_bits`

Converts a scalar value to a bit array of its bitlength.

Arguments:
- scalar value: `u{N}`, or `i{N}`, or `field`

Returns: `[bool; N]`

### `std::convert::from_bits_unsigned`

Converts a bit array to an unsigned integer of the array's bitlength.

Arguments:
- bit array: `[bool; N]`

Returns: `u{N}`

### `std::convert::from_bits_signed`

Converts a bit array to a signed integer of the array's bitlength.

Arguments:
- bit array: `[bool; N]`

Returns: `i{N}`

### `std::convert::from_bits_unsigned`

Converts a bit array to a field element.

Arguments:
- bit array: `[bool; 254]`

Returns: `field`

## `array` module

### `std::array::reverse`

Reverses the given array.

Arguments:
- array: `[{scalar}; N]`

Returns: `[{scalar}; N]`

### `std::array::truncate`

Truncates an array of size `N` to an array of size `new_length`.

Will cause a compile-error if either:
- `N` < `new_length`.
- `new_length` is not a constant expression

Arguments:
- array: `[{scalar}; N]`
- new_length: `u{N}` or `field`

Returns: `[{scalar}; new_length]`

### `std::array::pad`

Pads a given array with given values.

Will cause a compile-error if either:
- `N` < `new_length`.
- `new_length` is not a constant expression

Arguments:
- array: `[{scalar}; N]`
- new_length: `u{N}` or `field`
- fill_value: `{scalar}`

Returns: `[{scalar}; new_length]`
