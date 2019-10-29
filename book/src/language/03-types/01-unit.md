# Unit

No known differences from the Rust behavior.

`()` is the literal for both unit type and value. The unit type cannot be used
by any operators and cannot be casted to or from.

The unit type can exist as a standalone value:

```rust
let x = (); // ()
```

It can be returned by blocks or functions:

```rust
fn check(value: bool) {
    // several statements
};

let y = check(true); // ()
```
