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

A block can be modified with the `unconstrained` keyword, which turns on the
unconstrained mode in the Zinc VM. This mode is used to optimize the constraint
generation in computation-heavy code parts. Inside the block, constraints are
not creates, but instead, the block result must be constrained or included in
the application output.

```rust,no_run,noplaypen
let c = unconstrained {
    let a = 5;
    let b = 10;
    a + b
};
```
