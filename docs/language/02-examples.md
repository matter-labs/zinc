# Code examples

`simple_math.zrs`:

```rust
/// 
/// The example library.
/// 
/// Returns x^3.
/// 
pub fn cube(x: u128) -> u128 {
    let mut r = x;
    for i in 0..2 {
        r = r * x;
    }
    r
}
```

`main.zrs`:

```rust
///
/// The example binary.
/// 
/// Proves a knowledge of a cubic root `r` for a given public input `x`.
///

use simple_math;

inputs {
    x: u128,
}

witness {
    r: u128,
}

require(x == simple_math::cube(r), "x == r ^ 3");
```
