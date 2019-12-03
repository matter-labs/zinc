# Expression statements

## Expression

The expression statement is an expression terminated with a `;` in order
to ignore its result. The most common use is the assignment to a mutable
variable:

```rust,no_run,noplaypen
let mut a = 0;
a = 42; // an expression statement ignoring the '()' result of the assignment
```

For for information on expressions, check the **Chapter 6**.

## Semicolons

In contrast with Rust, expression statements must be always terminated with `;`
in Zinc to get rid of some ambiguities regarding block and conditional
expressions. Let us compare the examples of Rust and Zinc to illustrate the
problem.

```rust,no_run,noplaypen
fn blocks() -> i32 {
    {
        get_unit()
    } // a statement, but only because the block result is ()
    {
        get_integer()
    } // a return expression, only because the block result is an integer
}
```

In the Rust example above, the blocks are completely identical, but their semantic
meaning depends on the block return type. Zinc solves this problem by enforcing
all expression statements to be explicitly terminated with a semicolon, like in
the following Zinc example:

```rust,no_run,noplaypen
fn blocks() -> i32 {
    {
        get_unit()
    }; // a statement, because it is explicitly terminated with a semicolon
    {
        get_integer()
    } // a return expression, because it goes the last in the function block
}
```

Conditional and match expressions follow the same rules as simple blocks.
