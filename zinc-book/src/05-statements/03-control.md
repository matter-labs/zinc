# Control statements

Control statements neither ignore the result nor declare a new item. The
only such statement is the `for-while` loop.

## `for-while` loop

`for {identifier} in {integer}..{integer} [while {expression}] { ... }`

The `for` loop statement behaves just like in Rust, but it is merged with the
`while` loop, so the optional `while` condition is checked before each iteration
of the loop. The `while` condition expression has access to the inner scope and
can use its variables and the loop iterator.

```rust,no_run,noplaypen
for i in 0..10 while i % x != 8 {
    // do something
};
```

Only constant expressions can be used as the bounds of the iterator range. The
`while` condition will not cause an early return, but it will suppress the loop
body side effects.

Zinc is a turing-incomplete language, as it is dictated by R1CS restrictions, so
loops always have fixed number of iterations. On the one hand, the loop counter
can be optimized to be treated as a constant, reducing the circuit cost, but on
the other hand, you cannot force a loop to return early, increasing the circuit
cost.
