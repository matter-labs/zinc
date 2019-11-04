# Control statements

## Semicolon statement

The semicolon statement is an empty one and is just a `;`.

It is made mostly to keep backward-compatibility with presence or absence of
semicolons in some possible cases in the future.

## Expression statement

The expression statement is just an expression terminated with a `;` in order
to ignore its result. The most common use is the reassignment of the mutable
variable.

```rust
let mut a = 0; // declared a mutable variable 'a'
a = 42; // an expression statement ignoring the '()' result of the assignment
```

For for information on expressions, check the **Chapter 6**.
