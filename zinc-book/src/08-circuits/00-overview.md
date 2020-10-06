# Zero-knowledge circuits

A Zinc circuit consists of the entry point file called `main.zn` and zero or more
modules whose contents can be imported into the main file.

The entry point file must contain the `main` function, which accepts secret witness
data and returns public input data. For more detail, see the [next section](./01-input-output.md).

## Example

### Entry point file

```rust,no_run,noplaypen
//! 
//! 'src/main.zn'
//!
//! Proves a knowledge of a cube root `r` for a given public input `x`.
//!

mod simple_math;

fn main(x: field) -> field {
    simple_math::cube(x)
}
```

### Module `simple_math` file

```rust,no_run,noplaypen
//! 
//! 'src/simple_math.zn'
//! 

/// Returns x^3.
fn cube(x: field) -> field {
    x * x * x
}
```
