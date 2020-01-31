# Blocks

A block expression contains of zero or more statements and an optional result
expression. Each block has its own scope of visibility.

```rust,no_run,noplaypen
let c = {
    let a = 5;
    let b = 10;
    a + b
};
```
