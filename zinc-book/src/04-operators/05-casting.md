# Casting operator

`as` is a binary operator.

*Accepts*
1. Expression of any type
2. Expression of the same or another integer type

*Returns* the casted value.

Casting allowed:

- from integer to integer
- from enum to integer
- to the same type (no effect, no errors)

```rust,no_run,noplaypen
enum Order {
    First = 1,
}

let a = 1; // inferred as u8
let b = a as i8; // explicit casting to the opposite sign
let c: u8 = Order::First; // implicit casting to an integer
```
