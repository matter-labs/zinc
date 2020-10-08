# Control statements

Control statements neither ignore the result nor declare a new item. The
only such statement is the `for-while` loop.

## `for-while` loop

```rust,no_run,noplaypen
for {identifier} in {range} [while {expression}] {
    ...
}
```

The `for` loop statement can be modified with the `while` condition, which will
be checked before each iteration of the loop. The `while` condition expression
has access to the loop iterator variable.

```rust,no_run,noplaypen
let x = 7;

for i in 0..10 while i % x != 2 {
    // do something
};
```

Only constant expressions can be used as the bounds of the iterator range. The
`while` condition will not cause an early return, but it will suppress the loop
body side effects.

Zinc is a Turing-incomplete language, as it is dictated by R1CS restrictions, so
loops always have a fixed number of iterations. On the one hand, the loop counter
can be optimized to be treated as a constant, reducing the circuit cost, but on
the other hand, you cannot force a loop to return early, increasing the circuit
cost.

## `if` and `match`

The [conditional and match](../05-expressions/03-conditionals.md) expressions
can act as control statements, ignoring the returned value. To use them in such
a role, just terminate the expression with a semicolon:

```rust,no_run,noplaypen
fn unknown(value: u8) -> u8 {
    match value {
        1 => dbg!("One!"),
        2 => dbg!("Two!"),
        _ => dbg!("Perhaps, three!"),
    };
    42
}
```
