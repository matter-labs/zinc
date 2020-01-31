# Blocks

A block expression consists of zero or more statements and an optional result
expression. Every block starts a new scope of visibility.

```rust,no_run,noplaypen
let c = {
    let a = 5;
    let b = 10;
    a + b
};
```
