# Jabberwocky (Jab) circuit langauge spec

## Overview

The jab language will be used to simplify development of R1CS Zero-Knowledge circuits for generic computation. It will transpile into rust code based on the [bellman](https://github.com/matter-labs/bellman) library.

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

- field: native field element of the elliptic curve; represents an unsigned integer with 253..1024 bit length
- uint8 .. uint256: unsigned integers of different bitlength (with step 1)
- int8 .. int256: signed integers

### Boolean types

- bool: boolean values

### Vectors

- memory_vector<T>: array of elements of a given type in memory
- storage_vector<T>: array of elements of a given type in storage (tbd)

### Structs

- struct: grouping of elements of different types

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

- into_bits(): yields `memory_array<bool>`

### Supported operators for boolean types

- `&&`: logical and
- `||`: logical or
- `^^`: logical xor

## Statements

### Variable declaration

```rust
    let [mut] {var_name}: {type} = {expression};
```

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

## Expressions

tbd: arithmetic / boolean / mix

## Todo

### Unsorted

- linear combination optimizations
- value range meta information
- var naming
- var name scoping
- vector methods
- type inference

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
