# Debugging

There is a special `dbg!` function, which can print any data anywhere in your code.
The function prints data to the terminal and is used only for debugging purposes.

The first argument is the format string, where each `{}` placeholder is replaced
with a corresponding value from the rest of the arguments. The number of placeholders
must be equal to the number of the arguments not including the format string.

The full function description is [here](../appendix/D-intrinsic-functions.md).

## Example

```rust,no_run,noplaypen
// a = 5, b = 3
fn print_sum(a: u8, b: u8) {
    dbg!("{} + {} = {}", a, b, a + b); // prints '5 + 3 = 8'
}
```
