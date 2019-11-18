# Library

```rust,no_run,noplaypen
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
