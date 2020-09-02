# Expression statements

## Expression

The expression statement is an expression terminated with a `;` to ignore its
result. The most common use is the assignment to a mutable variable:

```rust,no_run,noplaypen
let mut a = 0;
a = 42; // an expression statement ignoring the '()' result of the assignment
```

For more information on expressions, check [this chapter](../07-expressions/00-overview.md).

## Semicolons

Expression statements in Zinc must be always terminated with `;` to get rid
of some ambiguities regarding block and conditional expressions.
