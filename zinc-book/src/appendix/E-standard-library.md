# Standard library

The standard library is unstable. Function signatures and behavior are going to
be changed in future releases.

## `std::sha256`

Computes the `sha256` hash of a given byte array.

Arguments:
- byte array (`[u8; N]`)

Return type: `[u8; 32]`

**Note**: This function is special, as it accepts a byte array of arbitrary size.

## `std::pedersen`

Computes the `pedersen` hash of a given byte array.

Arguments:
- byte array (`[u8; N]`)

Return type: `(field, field)`

**Note**: This function is special, as it accepts a byte array of arbitrary size.
