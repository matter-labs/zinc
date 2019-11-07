# Control statements

Control statements do neither ignore the result nor declare a new item. The
only such statement is the `for-while` loop.

## `for-while` loop

`for {identifier} in {integer}..{integer} [while {expression}] { ... }`

The `for` loop statement behaves just like in Rust, but it is merged with the
`while` loop, so the optional `while` condition is checked before each iteration
of the loop. The `while` condition expression has access to the inner scope and
can use its variables and the loop iterator.

```rust
for i in 0..10 while i % x != 8 {
    // do something
};
```

Only integer literals can be used as the bounds of the iterator range for now.
