# The standard library

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

## `std::crypto` module

### `std::crypto::sha256`

Computes the `sha256` hash of a given bit array.

Will cause a compile-error if either:
- preimage length is zero
- preimage length is not multiple of 8

Arguments:
- preimage bit array `[bool; N]`

Returns: 256-bit hash `[bool; 256]`

### `std::crypto::pedersen`

Maps a bit array to a point on an elliptic curve.

Will cause a compile-error if either:
- preimage length is zero
- preimage length is greater than 512 bits

To understand what is under the hood, see [this article](https://iden3-docs.readthedocs.io/en/latest/iden3_repos/research/publications/zkproof-standards-workshop-2/pedersen-hash/pedersen.html).

Arguments:
- preimage bit array `[bool; N]`

Returns: elliptic curve point coordinates `(field, field)`

### `std::crypto::ecc::Point`

The elliptic curve point.

```rust,no_run,noplaypen
struct Point {
    x: field,
    y: field,
}
```

### `std::crypto::schnorr::Signature`

The Schnorr EDDSA signature structure.

```rust,no_run,noplaypen
struct Signature {
    r: std::crypto::ecc::Point,
    s: field,
    pk: std::crypto::ecc::Point,
}
```

### `std::crypto::schnorr::Signature::verify`

Verifies the EDDSA signature.

Will cause a compile-error if either:
- message length is zero
- message length is greater than 248 bits

Arguments:
- the signature: `std::crypto::schnorr::Signature`
- the message: `[bool; N]`

Returns: the boolean result

## `std::convert` module

### `std::convert::to_bits`

Converts a scalar value to a bit array of its bitlength.

Arguments:
- scalar value: `u{N}`, or `i{N}`, or `field`

Returns: `[bool; N]`

### `std::convert::from_bits_unsigned`

Converts a bit array to an unsigned integer of the array's bitlength.

Will cause a compile-error if either:
- bit array size is zero
- bit array size is greater than 248 bits
- bit array size is not multiple of 8

Arguments:
- bit array: `[bool; N]`

Returns: `u{N}`

### `std::convert::from_bits_signed`

Converts a bit array to a signed integer of the array's bitlength.

Will cause a compile-error if either:
- bit array size is zero
- bit array size is greater than 248 bits
- bit array size is not multiple of 8

Arguments:
- bit array: `[bool; N]`

Returns: `i{N}`

### `std::convert::from_bits_unsigned`

Converts a bit array to a field element.

Arguments:
- bit array: `[bool; 254]`

Returns: `field`

## `std::array` module

### `std::array::reverse`

Reverses a given array.

Arguments:
- array: `[{scalar}; N]`

Returns: `[{scalar}; N]`

### `std::array::truncate`

Truncates an array of size `N` to an array of size `new_length`.

Will cause a compile-error if either:
- array size is less than new length
- new length is not a constant expression

Arguments:
- array: `[{scalar}; N]`
- new_length: `u{N}` or `field`

Returns: `[{scalar}; new_length]`

### `std::array::pad`

Pads a given array with the given values.

Will cause a compile-error if either:
- array size is greater than new length
- new length is not a constant expression

Arguments:
- array: `[{scalar}; N]`
- new_length: `u{N}` or `field`
- fill_value: `{scalar}`

Returns: `[{scalar}; new_length]`

## `std::ff` module

### `std::ff::invert`

Inverts a finite field.

Arguments:
- value: `field`

Returns: `field`

## `std::collections` module

### `std::collections::MTreeMap<K, V>`

The map type, which can only be a contract storage field and accessed
via the methods below.

### `std::collections::MTreeMap::get`

Gets the value from the map. Returns the value and presence flag.
If the presence flag is `false`, the value is filled with zeros.

Arguments:
- key: `K`

Returns: `(V, bool)`

### `std::collections::MTreeMap::contains`

Checks if the value exists in the map. Returns the presence flag.

Arguments:
- key: `K`

Returns: `bool`

### `std::collections::MTreeMap::insert`

Inserts the value into the map. Returns the old value and presence flag.
If the presence flag is `false`, the old value is filled with zeros.

Arguments:
- key: `K`
- value: `V`

Returns: `(V, bool)`

### `std::collections::MTreeMap::remove`

Removes the value from the map. Returns the removed value and presence flag.
If the presence flag is `false`, the removed value is filled with zeros.

Arguments:
- key: `K`

Returns: `(V, bool)`
