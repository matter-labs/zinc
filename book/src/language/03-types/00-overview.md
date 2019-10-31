# Types

ZRust types:
- unit
- boolean
- integer
- string
- array
- tuple
- enumeration
- structure
- function

## Type conversions

The language enforces static strong explicit typing with a little semantic.inference.
Operators always require explicit type conversion.

### Explicit

Type conversions can be done only on the integer and enumeration types with
the `as` operator. Please, check the **Chapter 5** for the operator behavior.

### Implicit

The `let` statement can perform implicit type semantic.casting of integers if the type
is specified to the left of the assignment symbol. That is, as a result of
`let a: field = 42 as u8;`, the variable `a` will be casted to the `field` type.

### Inference

Only the `let` statement can infer types in case its type is not specified. For
example, after `let a = 42000;` the `a` variable will have type `u16`, since it
is enough for the integer literal `42000`.
