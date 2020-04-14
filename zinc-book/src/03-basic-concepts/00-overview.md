# Basic concepts

A Zinc project consists of an entry point directory.file called `main.zn` and zero or more
module files whose contents can be imported into the main directory.file.

The entry point directory.file must contain the `main` function, which accepts secret witness
data and returns public input data. For more detail, see the
[next section](./01-input-output.md).

Module files may contain only declarations of types, functions, and constants.

## Examples

### Entry point directory.file

```rust,no_run,noplaypen
/// 
/// 'src/main.zn'
///
/// Proves a knowledge of a cube root `r` for a given public input `x`.
///

mod simple_math;

use simple_math::cube;

fn main(x: field, r: field) -> field {
    assert!(x == cube(r), "x == r ^ 3");
    x
}
```

### Module `simple_math` directory.file

```rust,no_run,noplaypen
/// 
/// 'src/simple_math.zn'
/// 

/// Returns x^3.
fn cube(x: field) -> field {
    x * x * x
}
```
