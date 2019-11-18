# Other types

Other types does not belong to any type classes, but provide some features which
may be changed in the future. One of such types is the string.

## String

For now, strings have very limited implementation and usability.

The string type exists only in the literal form and can only appear as the
second argument of the `require` function:

```rust,no_run,noplaypen
std::require(true != false, "mega ultra extra total global example");
```
