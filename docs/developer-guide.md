# Jabberwocky (Jab) developer guide

## Introduction

The jab language is used to simplify development of Quadratic Arithmetic Programs (see [this primer](http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/)). It converts a jab program into an R1CS circuit (a list of linbear constraints over finite fields) using the [bellman](https://github.com/matter-labs/bellman) library. This allows generation of Zero Knowledge Proofs for any proof system supported by bellman (such as Groth16 or Sonic).

:::info
Implementation details below are highlighted like this.
:::

## Circuit layout

`simple_math.jab`:

```rust
// returns a^3
pub fn cube(x: uint128) -> uint128 {
    let mut r = x;
    for i in 0..2 {
        r = r * x;
    }
    r
}
```

`main.jab`:

```rust
// program: prove a knowledge of a cubic root `r` for a given public input `x`

use simple_math;

inputs {
    x: uint128,
}

witness {
    r: uint128,
}

require(x == simple_math::cube(r), "x == r ^ 3");
```

## Module system and imports

Modules are defined hierarchically in files, following the rust cargo conventions. 

A module can be imported with the `use` keyword following the rust crate/module import rules:

```rust
use simple_math;
use simple_math::cube;
use simple_math::{cube, something_else};
use simple_math::*;
```

Only functions and types exposed in the libraries with `pub` keyword are imported.

Modules can be written in pure rust with bellman (tbd).

## Comments

Single line comments (`//`) and multi-line comments (`/*...*/`) are allowed and follow rust rules.

## Public inputs and secret witness

```rust
    input {
        {var_name}: {var_type},
        ...
    }

    witness {
        {var_name}: {var_type},
        ...
    }
```

Public inputs (defined in the `inputs` block) and secret witness (defined in the `witness` block) are the arguments of the program for which the circuit is implemented. The prover must provide both public inputs and secret witness data in order to generate proofs. The verifier must provide the same public input to verify the satisfiability of the proof.

Inputs and witness can only be defined once at the beginning of a circuit.

Variable names for input and witness are declared in the global variable namespace scope.

Each circuit must have 0 or more input arguments. It can have 0 or more witness arguments (if not arguments are provided, `witness` block can be omitted).

```rust
    input {
        x: uint128,
        ...
    }

    witness {
        cubic_root: uint128,
        ...
    }
```

## Statements

A statement can be on the following:

- variable declaration
- complex type definition
- control structure

Statments must be separated by a semicolon.

Following the rust convention, if the last statement in a block does not have a trailing semicolon, it returns a value. 

Since the circuit can not return a value, the last statement in the circuit programm must always have a trailing semicolon.

## Variable declaration

```rust
    let [mut] {var_name}: [{type}] = {expression};
```

Variables are immutable by default unless declared with `mut` keyword.

Reference declaration is not supported (yet).

Variable names follow the rust rules: they must begin with a symbol and can contain symbols, numbers and underscore.

Shadowing is allowed:

```rust
let x = 0;
let x = -1; // this is a different x with a different type
```

:::info
All variables must be named with scoping: scoping can be recursively introduced by conditionals and loops (tbd).

Variables will have the following meta-information collected by the compiler: 

- current variable (id or name) in the constraint system
- linear combination to compute the variable (which also includes representation of constant values)
- expected range: bit length which the user __promises__ to respect with regard to witness
- enforced range: bit length which is __guaranteed__ to have been enforced in the circuit
:::

## Types

### Primitive types

All primitive types must be initialized at declaration.

#### Native field type

`field` is a native field element of the elliptic curve used in the constraint system. It represents an unsigned integer of bit length equal to the field modulus length (e.g. for BN256 the field modulus length is 254 bit).

:::info
All other types are represented using `field` as their basic building block.
:::

#### Integer types

- `uint2` .. `uint{field_bit_length-1}`: unsigned integers of different bitlength (with step 1, e.g. for field length 254 the set will include [8, 9, ..., 252, 253])
- `int2` .. `int{field_bit_length-1}`: signed integers

`uint` and `int` without bits are synonyms for `uint{field_bit_length-1}` and `int{field_bit_length-1}` (largest representable integer values for the current curve).

:::info
When integers variables are allocated, their bitlength must be enforced in the constraint system.
:::

Integer literals:

- decimal: 0, 1, 122, -7 (inferred type: depending on the sign `uint`/`int` of the lowest possible bitlentgh)
- hexadecimal: 0x0, 0xfa, 0x0001 (inferred type: `uint` of the lowest possible bitlentgh)

```rust
let a = 0; // uint8
let a: int24 = 0; // int24
let b = 256; // uint16
let c = -1;  // int8
let c = -128; // int16
let d = 0xff; // uint8
let e: field = 0; // field
let f: uint = 0; // uint253 if the field modulus length is 254
```

#### Boolean type

- `bool`: boolean values

:::info
When `bool` variables are allocated, they must be enforced to only allow values `0` or `1`.
:::

Boolean literals:

- `true`
- `false`

```rust
let a = true;
let b: bool = false;
```

#### Enums

Jab supports simple enums (lists of constants), following the following restricted rust syntax:

```rust
enum Order {
    FIRST,
    SECOND,
}

let x = Order::FIRST;
let y: Order = x;
```

### Complex types

#### Tuples

Tuples follow the rust rules:

```rust
(uint8, field)
```

Like in rust, `()` represents the void value.

#### Structs

`struct` is a grouping of elements of different types. `struct` definitions follow the rust rules.

```rust
struct Person {
    age: uint8,
    id: uint64,
}
```

#### Fixed-sized arrays

Fixed-sized arrays follor the rust rules:

```rust
let fibbonaci: [uint8; 5] = [1, 1, 2, 3, 5];
let mut a: [field, 10]; // initialized with all zeros
```

##### Array functions: len(), reverse()

`len()` function of the shortest `uint` type possible:

```rust
let x = fibbonaci.len(); // let x: uint3 = 5;
```

`reverse()` produces a copy of the array in reverse order:

```rust
let a = [1, 2, 3];
let b = a.reverse(); // [3, 2, 1]
```

##### Access by index

Arrays support an index operator:

```rust
let fib3 = fibbonaci[3];
a[2] = 1;
```

__Note:__ accessing array by a constant index or `for` loop index is cheap, while accessing by a variable index incures linear cost of O(N), where N is array length.

##### Slicing

Using the rust slice syntax arrays can be transformed (producting a copy of the subarray -- by value, not by reference):

```rust
let a = [1, 2, 3, 4];
let b = a[1..2]; // [2, 3]
let b = a[1..]; // [2, 3, 4]
let b = a[..2]; // [1, 2, 3]
```

### Type conversions

#### Implicit type conversions

- any integer converts to `field`
- any `uint` can convert to a `int` of larger bitlength

```rust
let a = 1; // uint8
let b: int8 = a; // error: can not convert uint8 to int8
let c: int16 = a; // ok
let d = -1; // int8
let e: uint16 = d; // error: can not implicitly convert int to uint
let f: field = d; // ok
```

#### Explicit type coercions

If automatic conversion is not possible, user can coerce type conversions using `as` keyword (following the rust rules):

- any integer type can be coerced into another integer type of equal or greater bitlength without changes in underlying `field` value
- any integer type can be coerced into another integer type of lesser bitlength via bit decomposition (without range checks)
- an enum can be converted into an `uint` of enough bitlength

```rust
let a = -1; // int8
let b: uint8 = a as uint8; // ok
let c: uint8 = Order::FIRST; // ok
```

## Passing by value

Variables are always passed by value to operators, function calls and assignments. In R1CS programs passing by value is natural and cheap, albeit somewhat couner-intuitive: in fact, under the hood all variables are represented as references to immutable values, whereas change a mutable variable technically leads to creating a new variable with a reference to another immutable value.

In contrast to rust, passing by value in jab doesn't "move" the variable.

## Mutability

Variables declared with `mut` keyword can be reassigned.

```rust
let mut x = 0;
x = x + 1;
```

## Operators

### Scoping with parentheses

Parentheses (`(` and `)`) are used to introduce scoping for operations. Parentheses have highest priority of all operators.

### Operators for integer types

#### Types

Operators for integer types require implicit conversion of both operands to the result type according to the following rules:

|       |uint   |int    |field|
|uint   |uint   |int    |field|
|int    |int    |int    |field|
|field  |field  |field  |field|

Results of types `uint` and `int` have the bitlength of the largest operand (plus one bit for sign if operands are signed and unsigned).

#### Arithmetics

- `+`: addition
- `-`: subtraction
- `*`: multiplication

- `/`: integer division
- `%`: modulus
- `\`: inversion (for `field` type only)

Operators `+` and `-` increase the bitlength of the result by 1:

```rust
let a: uint8 = 1;
let b: uint8 = 1;
let c: int16 = 1;

let x = a + b; // uint9
let y = a + c; // int17
```

Operator `*` adds the bitlengths (not counting the sign bit).

```rust
let a: uint8 = 1;
let b: uint8 = 1;
let c: int16 = 1;

let x = a * b; // uint16
let y = a * c; // int24
let z = c * c; // int32
```

Operators `/`, `%` and `\` keep the bitlength of the result unchanged.

#### Range checks

Arithmetic operators will perform range checks on the results in two cases:

- whenever the bitlength of the result exceeds the field bitlength
- whenever the result is assigned to a type with smaller bitlength

It is possible to switch off range checks for `field` by placing the code in the `unsafe_unchecked` block:

```rust
let f: field = 0x1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

let sqr = unsafe_unchecked {
    f * f
};
```

#### Comparison

Operands will be converted to the common result type before the comparison.

Comparison always return a result of type `bool`.

- `==`
- `>`
- `<`
- `>=`
- `<=`

### Operators for boolean types

- `&&`: logical and
- `||`: logical or
- `^^`: logical xor

## Control structures

### Conditionals

```rust
    if {boolean_expression} {
        {statment};
        ...
    } [else {
        {statment};
        ...
    }];
```

:::info
- both branches are always executed
- conditionals create a name scope for variables
- all assignments inside a conditinal block are implemented as conditional assignments
- heavy function calls must be optimized with a stack (to explain in detail; this is tricky because it must be applied to the nested function calls)
:::

Conditional blocks can return value, following the rust rules:

```rust
let max = if a > b { 
    a 
} else { 
    b
};
```

Both branches must return the same type in this case.

### Loops

```rust
    for {var_name} in {range_start}..{range_end} [while {condition}] {
        {statement};
        ...
    }
```

`range_start` and `range_end` must be integer constants.

:::info
- loop create a name scope for variables on each loop cycle
:::

```rust
for i in 0..7 {
    // ...
}

for j in 15..1 while j > n {
    // ...
}
```

### Match

```rust
let square = match a {
    1 => { 1 },
    2 => 4,
    3 => 9,
    _ => painc("unexpected value"),
}
```

`match` follows the rust rules.

:::info
`match` will be implemented as a series of conditionals.
:::

## Functions

```rust
fn {function_name}({arguments}) [-> {result_type}] {
    {statement};
    ...
    {result_expression}
}
```

If the return type is omitted in the declaration, the function returns `()`.

The value is returned in the last statement without the trailing semicolon.

Not allowing returning the value in the middle of the function is a design decision to imply to the user that the function is always evaluated completely.

```rust
// calculate `x ^ y` for all `y` up to 8
fn pow(x: uint8, y: uint8) -> uint8 {
    require(y < 8);
    let r = 1;
    for i in 0..8 {
        if i < y {
            r = r * x;
        }
    };
    r 
}
```

Recursion is not supported.

## Embedded functions

### Require (consraint enforcement)

```rust
require(a == b); // automatically generates constraint named "a == b"
require(a == b, "a and b must be equal"); // custom name
```

### Debug traces

Jab provides an embedded `debug!()` macro which follows the rust format syntax:

```rust
debug!("a = {}, b = {}", a, b);
```

`debug!()` has no effect on constraint and witness generation and can only be used for debugging.

### into_bits / from_bits

Any primitive type and tuple can be converted to and from an array of `bool` bits.

```rust
// into_bits
let i: uint16 = 7;
let i_bits = i.into_bits(); // [bool; 16]
let x = (1 as uint64, 2 as uint10).into_bits(); // [bool; 74]

// from_bits
let slice = x[0..10];
let t: (uint8, bool, bool) = slice.from_bits();
```

## Standard library

- hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
  - hashes can accept array of bits of any length
- signatures: `eddsa_verify`
- curve primitives: `ecc`

## Reserved keywords

- input
- witness
- as
- mut
- for
- while
- break
- if
- struct
- fn
- return
- bool
- true
- false
- uint8...
- int8...
- unity
- pub
- use
