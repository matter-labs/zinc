# Boolean

`bool` is the boolean type keyword.

No known differences from the Rust behavior, but its internal representation
uses the elliptic curve field of bitlength `1`.

The type cannot be casted to or from.

## Literals

`true` and `false`

## Examples

```rust
let a = true;
let b: bool = false;

if a && !b {
    debug(a ^^ b);
};
```
