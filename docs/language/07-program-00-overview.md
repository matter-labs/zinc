# Program structure overview

## Public inputs and secret witness

```zrs
    input {
        {identifier}: {type},
        ...
    }

    witness {
        {identifier}: {type},
        ...
    }
```

Public inputs (defined in the `inputs` block) and secret witness (defined in the
`witness` block) are the arguments of the program for which the circuit is
implemented. The prover must provide both public inputs and secret witness data
in order to generate proofs. The verifier must provide the same public input
to verify the satisfiability of the proof.

Inputs and witness can only be defined once at the beginning of a circuit.

Variable names for input and witness are declared in the global variable namespace scope.

Each circuit must have 0 or more input arguments. It can have 0 or more witness
arguments (if not arguments are provided, `witness` block can be omitted).

```zrs
    input {
        x: u128,
        ...
    }

    witness {
        cubic_root: u128,
        ...
    }
```

## Functions (TODO)

```zrs
fn {identifier}({arguments})[ -> {type}] {
    {statement}*    
    {expression}
}
```

If the return type is omitted in the declaration, the function must return `()`.

The value is returned in the last statement without the trailing semicolon.

Not allowing returning the value in the middle of the function is a design
decision to imply to the user that the function is always evaluated completely.

```zrs
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

## Embedded functions (TODO)

### into_bits / from_bits

Any primitive type and tuple can be converted to and from an array of `bool` bits.

```zrs
// into_bits
let i: u16 = 7;
let i_bits = i.into_bits(); // [bool; 16]
let x = (1 as u64, 2 as u16).into_bits(); // [bool; 74]

// from_bits
let slice = x[0..10];
let t: (u8, bool, bool) = slice.from_bits();
```

## Standard library (TODO)

- hashes: `sha256`, `pedersen`, `poseidon`, `blake2s`
- signatures: `eddsa_verify`
- curve primitives: `ecc`
