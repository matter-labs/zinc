# Scalar types

Scalar types are also called primitive types and contain a single value.

## Unit

The unit type and value are described with empty round parenthesis `()` and
have no differences from the same type in Rust. Values of that type are
implicitly returned from functions, blocks, and other expressions which do not
return a value explicitly. Also, this type can be used as a placeholder for
input, witness and output types of the `main` function.

`()` is the literal for both unit type and value. The unit type values cannot be
used by any operators or casted back and forth.

The unit type can exist as a standalone value:

```rust
let x = (); // ()
```

It can be returned by blocks or functions:

```rust
fn check(value: bool) {
    // several statements
};

let y = check(true); // y is ()
```

## Boolean

`bool` is the boolean type keyword.

Its internal representation uses the BN256 field of bitlength `1`. But in
general, its behavior is indistinguishable from the same type from Rust or other
C-like languages.

Boolean values cannot be casted to or from.

### Literals

`true` and `false`.

### Examples

```rust
let a = true;
let b: bool = false;

if a && !b {
    debug(a ^^ b);
};
```

## Integer

Integer types are somewhat different from those of Rust, since they are
extended to be able to use any size between 1 and 32 bytes. This feature was
borrowed from Solidity and it helps to reduce the number of constraints and
smart contract size. Internal integer representation uses the BN256 field of
different bitlength.

### Types

- `u8` .. `u248`: unsigned integers
- `i8` .. `i248`: signed integers
- `field`: the native field integer

Integer types bitlength step equals 8, that is, only the following bitlengths
are possible: `8`, `16`, ..., `240`, `248`.

`field` is a native field element of the elliptic curve used in the constraint
system. It represents an unsigned integer of bitlength equal to the field
modulus length (e.g. for BN256 the field modulus length is `254` bit).

All the types are represented using `field` as their basic building block.
When integers variables are allocated, their bitlength must be enforced in the
constraint system.

### Literals

- decimal: `0`, `1`, `122`, `574839572494237242`
- hexadecimal: `0x0`, `0xfa`, `0x0001`, `0x1fffDEADffffffffffBEEFffff`

Following the Rust rules, only unsigned integer literals can be expressed, since
the unary minus is not a part of the literal but a standalone operator. Thus,
unsigned values can be implicitly casted to signed ones using the unary minus.

### Casting

Casting is possible only to a type with greater bitlength. Probably, this
behavior will become less strict in the future.

### Inference

If the literal type is not specified, the minimal possible bitlength is inferred.

### Examples

```rust
let a = 0; // u8
let a: i24 = 0; // i24
let b = 256; // u16
let c = -1;  // i8
let c = -129; // i16
let d = 0xff as field; // field
let e: field = 0; // field
```
