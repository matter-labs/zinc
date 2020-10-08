# Testing

The Zinc framework provides some basic unit testing functionality.

Unit tests are just simple functions marked with the `#[test]` attribute.
Such functions may be declared anywhere in the root scope of any module.

A test function can also be marked with other special attributes:

- `#[should_panic]` such test must fail in order to succeed, e.g. by passing a
false value to the `require` function or causing an overflow.

- `#[ignore]` such test is just ignored.

## Examples

```rust,no_run,noplaypen
#[test]
fn ordinar() {
    require(2 + 2 == 4, "The laws of the Universe have been broken");
}

#[test]
#[should_panic]
fn panicking() {
    require(2 + 2 == 5, "And it's okay");
}

#[test]
#[ignore]
fn ignored() {
    require(2 + 2 > 4, "So we'll just ignore it");
}
```
