# Casting operator

`as` is a binary operator.

**Accepts**
1. Expression of any type
2. Expression of the same type or integer type with equal or greater bitlength

**Returns** the integer result.

Casting allowed:

- from integers to types of equal or greater bitlength
- from enums to integers of enough or greater bitlength
- to the same type (no effect, no errors)

```rust,no_run,noplaypen
enum Order {
    First = 1,
}

let a = 1; // inferred as u8
let b = a as i8; // explicit casting to the opposite sign with the same bitlength 
let c: u8 = Order::First; // implicit casting to an integer of enough bitlength
```
