# Jabberwocky (Jab) circuit user guude

## Introduction

The jab language is used to simplify development of Quadratic Arithmetic Programs (see [this primer](http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/)). It converts a jab program into an R1CS circuit (a list of linbear constraints over finite fields) using the [bellman](https://github.com/matter-labs/bellman) library. This allows generation of Zero Knowledge Proofs for any proof system supported by bellman (such as Groth16 or Sonic).

:::info
Implementation details below are highlighted like this.
:::

## Circuit structure

```rust

// library: export a function
ciruit! {

    // returns a^3
    fn cube(x: uint128) -> uint128 {
        let mut r = x;
        for i in 0..2 {
            r = r * x;
        }
        return r
    }
}

// program: prove a knowledge of a cubic root `r` for a given public input `x`
ciruit! {

    inputs {
        x: uint128,
    }

    witness {
        r: uint128,
    }

    require(x == cube(r), "x == r ^ 3");
}
```

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

### Native field type

`field` is a native field element of the elliptic curve used in the constraint system. It represents an unsigned integer of bit length equal to the field modulus length (e.g. for BN256 the field modulus length is 254 bit).

:::info
All other types are represented using `field` as their basic building block.
:::

### Integer types

- `uint8` .. `uint{field_bit_length-1}`: unsigned integers of different bitlength (step 8 plus largest representable bitlength for the current field e.g. for field length 254 the set will include [8, 16, 24, ... 240, 248, 253])
- `int8` .. `int{field_bit_length-1}`: signed integers

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
```

### Boolean type

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

### Enums

Jab supports simple enums (lists of constants), following the following restricted rust syntax:

```rust
enum Order {
    FIRST,
    SECOND,
}

let x = Order::FIRST;
let y: Order = x;
```

### Structs

`struct` is a grouping of elements of different types. `struct` definitions follow the rust rules.

```rust
struct Person {
    age: uint8,
    id: uint64,
}
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

### Supported operators for boolean types

- `&&`: logical and
- `||`: logical or
- `^^`: logical xor

### Constraint enforcement

```rust
require(a == b); // automatically generates constraint named "a == b"
require(a == b, "a and b must be equal"); // custom name
```

### Conditionals

```rust
    if {boolean_expression} {
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

`range_start` and `range_end` must be integer constants. `range_end` must be greater or equal to `range_start`.

## Expressions

tbd: arithmetic / boolean / mix

:::info
TODO: explain optimizations of linear combinations
:::

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

## To define

- comments
- code inclusion
- string<length>
- bytes?
- literals: dec, hex, string, bool, etc?
- enum
- functions

### State

- contract variables

### Unsorted

- modular division, invesrsion
- exponentiation
- debug trace
- bitshift ops
- recursion

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

### Vectors

- `memory_vector<T, size>`: fixed-sized array of elements of a given type in memory
- `storage_vector<T, size>`: fixed-sized array of elements of a given type in storage (tbd)

__Implementation details:__ vectors with random index access can have different implementations depending on the vector size and the way it is used. Possible implementations:

- Merkle tree
- Linear scan

### Supported operators for vectors

- `[c]`: access element by index `c` (where `c` is a constant)
- `[i]`: access element by index `i` (where `i` is a integer variable)

**Embedded methods**:

- `into_bits()`: yields `memory_array<bool, bit_length>`

### Reserved keywords

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
- bool
- true
- false
- uint8...
- int8...
