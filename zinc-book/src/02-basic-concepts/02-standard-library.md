# Standard library

The standard library is currently located in a built-in module called `std`.
The library contains only two hash functions for now: `sha256` and `pedersen`.

The function signatures are listed in **Appendix E**.

Standard library items can be used directly or be imported with `use`:

```rust,no_run,noplaypen
use std::sha256;

fn main(preimage: [u8; 256]) -> ([u8; 32], (field, field)) {
    let input_sha256 = sha256(preimage); // with import
    dbg!(input_sha256);

    let input_pedersen = std::pedersen(preimage); // directly
    dbg!(input_pedersen);

    (input_sha256, input_pedersen)
}
```

## Built-in functions

Built-in functions closely resemble `macro_rules` found in Rust, but you do not
have to declare them since the compiler already knows all signatures.

The exhaustive list of function signatures is provided in **Appendix D**.

To call such a function, use the Rust macro syntax, as in the following example:

```rust,no_run,noplaypen
fn main(/* ... */) {
    let value: u8 = 42;
    dbg!("{}", value);
    assert!(value == 42);
}
```
