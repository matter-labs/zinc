# Expression statements

## Empty

The semicolon statement `;` is an empty statement.

It is made mostly to keep backward-compatibility with presence or absence of
semicolons in some possible cases in the future.

## Expression

The expression statement is an expression terminated with a `;` in order
to ignore its result. The most common use is the assignment to a mutable
variable:

```rust
let mut a = 0;
a = 42; // an expression statement ignoring the '()' result of the assignment
```

For for information on expressions, check the **Chapter 6**.

## Semicolons

In contrast with Rust, expression statements must be always terminated with `;`
in ZRust to get rid of some ambiguities regarding block and conditional
expressions. Let us compare the examples of Rust and ZRust to illustrate the
problem.

```rust
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
meaning depends on the block return type. ZRust solves this problem by enforcing
all expression statements to be explictly terminated with a semicolon, like in
the following ZRust example:

```rust
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
