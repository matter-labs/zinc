# Casting operator

`as` is a binary operator.

**Accepts**
1. Integer expression (anything but `field`)
2. Integer type of the equal or greater bitlength

**Returns** the integer result.

Casting allowed:

- from integers to types of equal or greater bitlength
- from enums to integers of enough or greater bitlength

```rust,no_run,noplaypen
let a = -1; // inference
let b: u16 = a as u16; // ok, casted to the opposite sign with greater bitlength 
let c: u8 = Order::First; // casting to an integer of enough bitlength
```
