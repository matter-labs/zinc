# Binary

```rust,no_run,noplaypen
/// 
/// './main.zn'
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
