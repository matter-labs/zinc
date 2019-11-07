# Compound types

Compound types consist of several fields bound together. There are two such types
in ZRust: arrays and tuples.

## Array

Arrays are collections of values of the same type sequentially stored in the memory.

Fixed-sized arrays follow the Rust rules. The only exception is the restriction
to constant indexes, that is, you cannot index an array with anything but an
integer literal for now.

Arrays support the index operator, which is explained in detail in **Chapter 5**.

```rust
let element = fibbonaci[3];
fibbonaci[2] = 1;
```

## Tuple

Tuples are anonymous collections of values of different types, sequentially
stored in memory and gathered together due to some logical relations.

Like in Rust, `()` is the void value, `(value)` is a parenthesized expression,
and `(value,)` is a tuple with one element.

Tuple fields can be accessed via the dot operator, which is explained in details
in **Chapter 5**.

```rust
let mut tuple: (u8, field) = (0xff, 0 as field);
tuple.0 = 42;
debug(tuple.1);
```
