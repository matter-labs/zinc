# Types

## Primitive types

### Unit

`()` is the unit type and value.

It behaves just like in Rust and is returned implicitly by blocks which do not
return a value explicitly.

The type cannot be used in any expressions or casting operations.

#### Examples

```jab
let x: () = ();
let y = {
    debug!(42);
};
```

### Boolean

`bool` is the boolean type keyword

#### Boolean literals

- `true`
- `false`

#### Examples

```jab
let a = true;
let b: bool = false;
```

### Integer

- `u8` .. `u248`: unsigned integer keywords with different bitlength
- `i8` .. `i248`: signed integer keywords with different bitlength
- `field`: the native field unsigned integer

Integer types bitlength step equals 8, that is, only the following bitlengths
are possible: `8`, `16`, ..., `240`, `248`.

`field` is a native field element of the elliptic curve used in the constraint
system. It represents an unsigned integer of bitlength equal to the field
modulus length (e.g. for BN256 the field modulus length is `254` bit).

All other types are represented using `field` as their basic building block.

When integers variables are allocated, their bitlength must be enforced in the constraint system.

#### Integer literals

- decimal: `0`, `1`, `122`, `574839572494237242`
- hexadecimal: `0x0`, `0xfa`, `0x0001`, `0x1fffDEADffffffffffBEEFffff`

Following the Rust rules, only unsigned integer literals can be expressed, since
the unary minus is not a part of the literal but a standalone operator. Thus,
unsigned values can be implicitly casted to signed ones using the unary minus.

#### Integer type inference

If the literal type is not specified, the minimal possible bitlength is inferred.

`let` statement performs an implicit casting if the type is specified.

#### Examples

```jab
let a = 0; // u8
let a: i24 = 0; // i24
let b = 256; // u16
let c = -1;  // i8
let c = -129; // i16
let d = 0xff as field; // field
let e: field = 0; // field
```

#### Integer casting

Casting is possible only to a type with greater bitlength. Probably, this
behavior will become less strict in the future.

### String

The string type exists only in the literal form and can only appear as the
second argument of the `require` statement.

```jab
require(true != false, "mega ultra extra total global example");
```

## Complex types

### Fixed-size array

Fixed-sized arrays follow the Rust rules:

```jab
let fibbonaci: [u8; 5] = [1, 1, 2, 3, 5];
let mut a: [field, 10]; // initialized with all zeros
```

The only exception is the temporary restriction to constant indexes, that is,
you cannot index an array with a variable for now.

#### Indexing an array

Arrays support an index operator:

```jab
let element = fibbonaci[3];
fibbonaci[2] = 1;
```

### Tuple

Tuples follow the Rust rules:

```jab
let tuple: (u8, field) = (0xff, 0 as field);
```

Like in Rust, `()` is the void value, `(value)` is a parenthesized expression,
and `(value,)` is a tuple with one element.

### Enum

Simple C-like enums are supported, following the restricted Rust syntax:

```jab
enum Order {
    FIRST, // 0
    SECOND, // 1
}

let x = Order::FIRST; // Order
let y: u8 = Order::SECOND; // u8
```

### Structure

Structure definitions and behavior follow the Rust rules.

```jab
struct Person {
    age: u8,
    id: u64,
};
```

## Type conversions

The language enforces static strong explicit typing with a little inference.
Operators almost always require explicit type conversion.

Only the `let` statement can infer types for now.

Casting can be performed using `as` keyword (following the Rust rules):

- integers to types of greater bitlength
- enums can be implicitly converted to unsigned integers of enough bitlength

```jab
let a = -1; // `i8`, after a cast with the unary minus and the `let` inference
let b: u16 = a as u16; // ok, casted to greater bitlength 
let c: u8 = Order::FIRST; // ok, enum implicit casting to enough bitlength
```
