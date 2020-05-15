# Constant expressions

A constant expression is evaluated at compile time. It is useful to declare
some global data used throughout the application.

```rust,no_run,noplaypen
const UNIX_EPOCH_TIMESTAMP: (u64, u8, u8) = (1970, 1, 1);
```
