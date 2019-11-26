# Standard library

The standard library is currently limited to several built-in functions embedded
into the ZRust VM. 

## Built-in functions

Built-in functions closely resemble `macro_rules` found in Rust, but you do not
have to declare them, since the compiler already knows all the built-in function
signatures.

Among these functions are:
- helper ones: `dbg`, `assert` etc.
- hashing ones: `sha256`, `pedersen`, `rescue` etc.

The exhaustive list of function signatures is provided in the appendix D.

To call such a function, use the Rust macro syntax, as in the following example:

```rust
fn main(input: field) {
    let input_sha256 = sha256!(input);
    dbg!(input_sha256);

    let input_pedersen = pedersen!(input);
    dbg!(input_pedersen);

    let input_rescue = rescue!(input);
    assert!(input_rescue == SOME_RESULT);
}
```
