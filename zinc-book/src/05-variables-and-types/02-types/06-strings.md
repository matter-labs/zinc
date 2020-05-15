# Strings

For now, strings have very limited implementation and usability.

The string type exists only in the literal form and can only appear in the
`dbg` and `assert` built-in functions:

```rust,no_run,noplaypen
dbg!("{}", 42);

assert!(true != false, "a very obvious fact");
```
