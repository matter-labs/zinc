# The `require` function

This function creates a custom constraint in any place of your code.
Using `require()` you can check whether some condition is true
and make the application exit with an error if otherwise.

The full function description is [here](../appendix/D-intrinsic-functions.md).

## Example

```rust,no_run,noplaypen
const BAD_VALUE: u8 = 42;

fn wrong(a: u8, b: u8) -> u8 {
    let c = a + b - BAD_VALUE;
    require(a + b == c, "always fails");
    c
}
```
