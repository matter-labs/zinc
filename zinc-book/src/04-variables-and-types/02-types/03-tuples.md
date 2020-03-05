# Tuples

Tuples are anonymous collections of values of different types, sequentially
stored in memory and gathered together due to some logical relations.

> Like in Rust, `()` is a void value, `(value)` is a parenthesized expression,
> and `(value,)` is a tuple with one element.

Tuple fields can be accessed via the dot operator, which is explained in detail [here](../../05-operators/05-access.md).

```rust,no_run,noplaypen
let mut tuple: (u8, field) = (0xff, 0 as field);
tuple.0 = 42;
dbg!("{}", tuple.1);
```
