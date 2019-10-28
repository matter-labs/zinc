# Examples

## Library

```rust
/// 
/// './lib.rs'
/// 
/// Returns x^3.
/// 
fn cube(x: u128) -> u128 {
    let mut r = x;
    for i in 0..2 {
        r = r * x;
    }
    r
}
```

## Binary

```rust
/// 
/// './main.rs'
///
/// Proves a knowledge of a cubic root `r` for a given public input `x`.
///

mod simple_math;

use simple_math::cube;

input {
    x: u128,
}

witness {
    r: u128,
}

require(x == cube(r), "x == r ^ 3");
```
