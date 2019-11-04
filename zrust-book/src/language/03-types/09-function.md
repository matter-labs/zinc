# Function

Functions does not yet accept references and the `return` statement.

```rust
fn sum(a: u8, b: u8) -> u8 {
    a + b
};

let result = sum(42, 27);
require(result == 69);
```
