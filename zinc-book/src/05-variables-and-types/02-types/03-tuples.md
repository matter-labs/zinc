# Tuples

Tuples are anonymous collections of values of different types, sequentially
stored in memory and gathered together due to some logical relations.

Tuple fields can be accessed via the dot operator, which is explained in detail [here](../../06-operators/06-access.md).

```rust,no_run,noplaypen
let mut tuple: (u8, field) = (0xff, 0 as field);
tuple.0 = 42;
dbg!("{}", tuple.1);
```

> If you familiar with Rust, you may remember the peculiar connection between
> unit values, parenthesized expressions, and tuples of one element:
> - `()` is a unit value
> - `(value)` is a parenthesized expression
> - `(value,)` is a tuple of one element
