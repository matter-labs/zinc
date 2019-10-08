## Module system and imports

Modules are defined hierarchically in files, following the Rust cargo conventions. 

A module can be imported with the `use` keyword following the Rust crate/module import rules:

```jab
use simple_math;
use simple_math::cube;
use simple_math::{cube, something_else};
use simple_math::*;
```

Only functions and types exposed in the libraries with `pub` keyword are imported.

Modules can be written in pure Rust with bellman (tbd).

## Comments

Single line comments (`//`) and multi-line comments (`/*...*/`) are allowed and follow Rust rules.

## Public inputs and secret witness

```jab
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

```jab
    input {
        x: u128,
        ...
    }

    witness {
        cubic_root: u128,
        ...
    }
```

## Statements

A statement can be on the following:

- variable declaration
- complex type definition
- loop control structure
- semicolon terminated expression
- built-in functions like `debug(...)` or `require(...)` until they are implemented as genuine functions

Statements must be separated by a semicolon.

Following the Rust convention, if the last statement in a block does not have a trailing semicolon, it returns a value. 

Since the circuit cannot return a value, the last statement in the circuit program must always have a trailing semicolon.

## Variable declaration

```jab
    let [mut] {identifier}[: {type}] = {expression};
```

Variables are immutable by default unless declared with `mut` keyword.

Reference declaration is not supported (yet).

Variable names follow the Rust rules: they must begin with a symbol and can contain symbols, numbers and underscore.

Shadowing is allowed:

```jab
let x = 0;
let x = -1; // this is a different x with a different type
```

:::info
All variables must be named with scoping: scoping can be recursively introduced by conditionals and loops.

Variables will have the following meta-information collected by the compiler:

- current variable (id or name) in the constraint system
- linear combination to compute the variable (which also includes representation of constant values)
- expected range: bit length which the user __promises__ to respect with regard to witness
- enforced range: bit length which is __guaranteed__ to have been enforced in the circuit
:::

## Passing by value

Variables are always passed by value to operators, function calls and assignments. In R1S programs passing by value is natural and cheap, albeit somewhat couner-intuitive: in fact, under the hood all variables are represented as references to immutable values, whereas change a mutable variable technically leads to creating a new variable with a reference to another immutable value.

In contrast to Rust, passing by value in jab doesn't "move" the variable.

## Mutability

Variables declared with `mut` keyword can be reassigned.

```jab
let mut x = 0;
x = x + 1;
```

## Operators

### Scoping with parentheses

Parentheses (`(` and `)`) are used to introduce scoping for operations. Parentheses have highest priority of all expression tokens.

### Operators for integer types

#### Types

Jab requires strong typing. Operators on operands of different types require explicit type conversion.

#### Arithmetics

- `+`: addition
- `-`: subtraction
- `*`: multiplication

- `/`: integer division
- `%`: modulus
- `\`: inversion (for `field` type only) (tbd)

#### Range checks

When the expression is computed, the expected bitlength of the result must be extended as long as it fits the field. 

Arithmetic operators will perform range checks (and bit adjustment of the result) in two cases:

- whenever the bitlength of the result exceeds the field bitlength
- whenever the result is assigned to a type with smaller bitlength

It is possible to switch off range checks for `field` by placing the code in the `unsafe_unchecked` block:

```jab
let f: field = 0x1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

let sqr = unsafe_unchecked {
    f * f
};
```

#### Comparison

Operands will be converted to the common result type before the comparison.

Comparison always return a result of type `bool`.

- `==`
- `!=`
- `>`
- `<`
- `>=`
- `<=`

### Operators for boolean types

- `!`: logical not
- `&&`: logical and
- `||`: logical or
- `^^`: logical xor

## Control structures

### Conditionals

```jab
    if {boolean_expression} {
        {statement};*
        {expression}?
    } [else if {boolean_expression} {
        {statement};*
        {expression}?
    }] [else {
        {statement};*
        {expression}?
    }];
```

:::info
- both branches are always executed
- conditionals create a name scope for variables
- all assignments inside a conditinal block are implemented as conditional assignments
- heavy function calls must be optimized with a stack (to explain in detail; this is tricky because it must be applied to the nested function calls)
:::

Conditional blocks can return value, following the Rust rules:

```jab
let max = if a > b { 
    a 
} else { 
    b
};
```

Both branches must return the same type in this case.

### Loops

```jab
    for {var_name} in {range_start}..{range_end} [while {condition}] {
        {statement};
        ...
    }
```

`range_start` and `range_end` must be integer constants.

:::info
- loop create a name scope for variables on each loop cycle
:::

```jab
for i in 0..7 while i > n {
    // ...
}
```

### Match

```jab
let square = match a {
    1 => { 1 },
    2 => 4,
    3 => 9,
    _ => panic("unexpected value"),
}
```

`match` follows the Rust rules.

:::info
`match` will be implemented as a series of conditionals.
:::

## Functions

```jab
fn {function_name}({arguments}) [-> {result_type}] {
    {statement}*    
    {expression}
}
```

If the return type is omitted in the declaration, the function returns `()`.

The value is returned in the last statement without the trailing semicolon.

Not allowing returning the value in the middle of the function is a design decision to imply to the user that the function is always evaluated completely.

```jab
// calculate `x ^ y` for all `y` up to 8
fn pow(x: u8, y: u8) -> u8 {
    require(y < 8);
    let r = 1;
    for i in 0..8 {
        if i < y {
            r = r * x;
        };
    };
    r 
}
```

Recursion is not supported.

## Embedded functions

### Require (consraint enforcement)

```jab
require(a == b); // automatically generates constraint named "a == b"
require(a == b, "a and b must be equal"); // custom name
```

### Debug traces

Jab provides an embedded `debug!()` macro which follows the Rust format syntax:

```jab
debug!("a = {}, b = {}", a, b);
```

`debug!()` has no effect on constraint and witness generation and can only be used for debugging.

### into_bits / from_bits

Any primitive type and tuple can be converted to and from an array of `bool` bits.

```jab
// into_bits
let i: u16 = 7;
let i_bits = i.into_bits(); // [bool; 16]
let x = (1 as u64, 2 as u16).into_bits(); // [bool; 74]

// from_bits
let slice = x[0..10];
let t: (u8, bool, bool) = slice.from_bits();
```

## Standard library

- hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
  - hashes can accept array of bits of any length
- signatures: `eddsa_verify`
- curve primitives: `ecc`
