# Integer

Integer types are somewhat different from those of Rust.

## Types

- `u8` .. `u248`: unsigned integers
- `i8` .. `i248`: signed integers
- `field`: the native field integer

Integer types bitlength step equals 8, that is, only the following bitlengths
are possible: `8`, `16`, ..., `240`, `248`.

`field` is a native field element of the elliptic curve used in the constraint
system. It represents an unsigned integer of bitlength equal to the field
modulus length (e.g. for BN256 the field modulus length is `254` bit).

All other types are represented using `field` as their basic building block.

When integers variables are allocated, their bitlength must be enforced in the constraint system.

## Literals

- decimal: `0`, `1`, `122`, `574839572494237242`
- hexadecimal: `0x0`, `0xfa`, `0x0001`, `0x1fffDEADffffffffffBEEFffff`

Following the Rust rules, only unsigned integer literals can be expressed, since
the unary minus is not a part of the literal but a standalone operator. Thus,
unsigned values can be implicitly casted to signed ones using the unary minus.

## Casting

Casting is possible only to a type with greater bitlength. Probably, this
behavior will become less strict in the future.

## Inference

If the literal type is not specified, the minimal possible bitlength is inferred.

The `let` statement performs an implicit semantic.casting if the type is specified.

## Examples

```rust
let a = 0; // u8
let a: i24 = 0; // i24
let b = 256; // u16
let c = -1;  // i8
let c = -129; // i16
let d = 0xff as field; // field
let e: field = 0; // field
```
