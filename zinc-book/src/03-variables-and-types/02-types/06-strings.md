# Strings

For now, strings have very limited implementation and usability.

The string values may exist only in the literal form and can only appear in the
`dbg` and `require` intrinsic functions:

```rust,no_run,noplaypen
dbg!("{}", 42); // format string

require(true != false, "a very obvious fact"); // optional error message
```
