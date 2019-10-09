# Overview

## Variables

Variables will have the following meta-information collected by the compiler:

- current variable (id or name) in the constraint system
- linear combination to compute the variable (which also includes representation of constant values)
- expected range: bit length which the user **promises** to respect with regard to witness
- enforced range: bit length which is **guaranteed** to have been enforced in the circuit

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
