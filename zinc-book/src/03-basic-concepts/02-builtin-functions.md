# Built-in functions

There are several built-in functions, which can be called directly from
anywhere in your code.

## `assert!()`

This function creates a custom constraint in any place of your code.
Using `assert!()` you can check whether some condition is true
and make the circuit exit with an error if otherwise:

```rust,no_run,noplaypen
fn wrong(a: u8, b: u8) -> u8 {
    let c = a + b - BAD_VALUE;
    assert!(a + b == c, "always fails");
    c
}
```

## `dbg!()`

This function prints data to the terminal and is used only for debugging purposes.

```rust,no_run,noplaypen
fn print_sum(a: u8, b: u8) {
    dbg!("{} + {} == {}", a, b, a + b);
}
```

## Example

To call such a function, use the `<identifier>!(arg1, arg2, ...)` syntax,
as in the following example:

```rust,no_run,noplaypen
fn main(/* ... */) {
    let value: u8 = 42;
    dbg!("{}", value);
    assert!(value == 42);
}
```

If you are familiar with Rust, it can resemble the macro syntax found there, but
actually, these functions have nothing to do with macros. Instead, they
represent some special Zinc VM instructions.

The exhaustive list of function signatures is provided in [Appendix D](../appendix/D-built-in-functions.md).
