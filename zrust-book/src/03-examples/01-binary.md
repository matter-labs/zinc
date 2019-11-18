# Binary

```rust,no_run,noplaypen
/// 
/// './main.rs'
///
/// Proves a knowledge of a cubic root `r` for a given public input `x`.
///

mod simple_math;

use simple_math::cube;

struct Input {
    x: u128,
}

struct Witness {
    r: u128,
}

struct Output {}

fn main(input: Input, witness: Witness) -> Output {
    require(x == cube(r), "x == r ^ 3");
    Output {}
}
```
