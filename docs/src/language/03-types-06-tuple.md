# Tuple

No known differences from the Rust behavior.

Like in Rust, `()` is the void value, `(value)` is a parenthesized expression,
and `(value,)` is a tuple with one element.

## Examples

```rust
let mut tuple: (u8, field) = (0xff, 0 as field);
tuple.0 = 42;
debug(tuple.1);
```
