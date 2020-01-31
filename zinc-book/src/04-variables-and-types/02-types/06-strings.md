# Other types

Other types do not belong to any type class but provide some features which
may be changed in the future. One such type is the string.

## String

For now, strings have very limited implementation and usability.

The string type exists only in the literal form and can only appear in `dbg` and
`assert` built-in functions:

```rust,no_run,noplaypen
dbg!("{}", 42);

assert!(true != false, "a very obvious fact");
```
