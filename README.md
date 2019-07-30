# Jabberwocky (Jab) circuit langauge spec

## Overview

The jab language will be used to simplify development of R1CS (see [this primer](http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/)) Zero-Knowledge circuits for generic computation. It will transpile into rust code based on the [bellman](https://github.com/matter-labs/bellman) library.

## Compiler

Compiler v1 must transpile rust files with `circuit!` macro invocation into expanded rust files (where the macro contents is replaced with rust code).

## Program structure

```rust

ciruit! {

    inputs {
        {input_name}: {type},
        ...
    }

    [witness {
        {witness_var_name}: {type},
        ...
    }]

    {statement};
    ...
}

```

## Types

### Integer types

- `field`: native field element of the elliptic curve; represents an unsigned integer with 253 or more bit length. Field is defined once for each constraint system.
- `uint8` .. `uint{field_bit_length}`: unsigned integers of different bitlength (with step 1)
- `int8` .. `int{field_bit_length}`: signed integers

__Implementation details:__ all integers are reprsented as `field` under the hood.

### Boolean types

- `bool`: boolean values

__Implementation details:__ all integers are reprsented as `field` under the hood, which is enforced to only allow values `0` or `1`.

### Vectors

- `memory_vector<T, size>`: fixed-sized array of elements of a given type in memory
- `storage_vector<T, size>`: fixed-sized array of elements of a given type in storage (tbd)

__Implementation details:__ vectors with random index access can have different implementations depending on the vector size and the way it is used. Possible implementations:

- Merkle tree
- Linear scan

### Structs

- `struct`: grouping of elements of different types

## Operators

### Scoping with parentheses

Parentheses (`(` and `)`) are used to introduce scoping for operations. Parentheses have highest priority of all operators.

### Supported operators for integer types

**Arithmetics** (yield results of the greatest bit length of the operands):

- `+`: addition
- `-`: subtraction
- `*`: multiplication
- `/`: inversion

Arithmetic operators must perform range checks on the results.

**Comparison** (always yield `bool`):

- `==`
- `>`
- `<`
- `>=`
- `<=`

**Embedded methods**:

- `into_bits()`: yields `memory_array<bool, bit_length>`

### Supported operators for boolean types

- `&&`: logical and
- `||`: logical or
- `^^`: logical xor

### Supported operators for vectors

- `[c]`: access element by index `c` (where `c` is a constant)
- `[i]`: access element by index `i` (where `i` is a integer variable)

## Type conversions

tbd

## Statements

### Variable declaration

```rust
    let [mut] {var_name}: [{type}] = {expression};
```

Variables are immutable by default unless declared with `mut` keyword.

All variables must be named with scoping: scoping can be recursively introduced by conditionals and loops (tbd).

__Implementation details__: variables will have the following meta-information collected by the compiler: 

- current variable (id or name) in the constraint system
- linear combination to compute the variable (which also includes representation of constant values)
- expected bit length: bit length which the user __promises__ to respect with regard to witness
- enforced range: bit length which is __guaranteed__ to have been enforced in the circuit

### Constraint enforcement

```rust
    require({boolean_expression});
```

### Conditionals

```rust
    if {boolean_statement} {
        {statment};
        ...
    } [else {
        {statment};
        ...
    }]
```

### Loops

```rust
    for {var_name} in {range_start}..{range_end} {
        {statement};
        ...
    }
```

`range_start` and `range_end` MUST be integer constants. `range_end` MUST be greater or equal to `range_start`.

## Expressions

tbd: arithmetic / boolean / mix

## Todo

### Vars

- linear combination optimizations
- value range meta information
- when to do range checks
- var naming
- var name scoping
- vector methods
- `vector<bool, size>.pack()`
- type inference
- type conversions

### Unsorted

- modular division, invesrsion
- exponentiation
- debug trace
- bitshift ops
- literals: dec, hex

### Bellman gadgets

- sha256
- pedersen
- sig_verify

### Code conversion samples

- [ ] inputs: `inputs { a: type, b: type }`
- [ ] witness: `witness { a: type, b: type }`
- [ ] witness generators: `unsafe_witness { /* bellman/rust code */ }`
- [ ] types: `let [mut] a: {type} = {value};`
- [ ] operators: LCs, ranges, overflow checks => range check on assignment
- [ ] require: `require({boolean condition});`
- [ ] if: conditional assignments, computational reuse optimizations
- [ ] for: constant range
- [ ] struct: assignments
- [ ] functions / gadgets
- [ ] unsafe_witness{} code

### Optimizations

- [ ] conditional accumulation of heavy functions

### Formal

- [ ] Formal language spec
