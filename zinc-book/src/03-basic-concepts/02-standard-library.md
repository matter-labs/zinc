# Standard library

The standard library is currently located in a built-in module called `std`.
The library contains three modules for now:
- `crypto` - cryptographics and hash functions
- `convert` - bit array conversion functions
- `array` - array processing functions

All the function signatures are listed in [Appendix E](../appendix/E-standard-library.md).

Standard library items can be used directly or be imported with `use`:

```rust,no_run,noplaypen
use std::crypto::sha256;

fn main(preimage: [bool; 256]) -> ([bool; 256], (field, field)) {
    let input_sha256 = sha256(preimage); // through import
    dbg!(input_sha256);

    let input_pedersen = std::crypto::pedersen(preimage); // directly
    dbg!(input_pedersen);

    (input_sha256, input_pedersen)
}
```

## Built-in functions

Built-in functions closely resemble `macro_rules` found in Rust, but you do not
have to declare them since the compiler already knows all their signatures.

The exhaustive list of function signatures is provided in [Appendix D](../appendix/D-built-in-functions.md).

To call such a function, use the Rust macro syntax, as in the following example:

```rust,no_run,noplaypen
fn main(/* ... */) {
    let value: u8 = 42;
    dbg!("{}", value);
    assert!(value == 42);
}
```
