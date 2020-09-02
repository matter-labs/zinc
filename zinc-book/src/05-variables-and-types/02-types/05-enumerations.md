# Enumerations

These allow you to define a type by enumerating its possible values. Only simple
C-like enums are supported for now, which are groups of constants:

```rust,no_run,noplaypen
enum Order {
    FIRST = 0,
    SECOND = 1,
}
```

Enum values can be used with `match` expressions to define the behavior in every
possible case:

```rust,no_run,noplaypen
let value = Order::FIRST;
let result = match value {
    Order::FIRST => do_this(),
    Order::SECOND => do_that(),
};
```

The enum values can be implicitly casted to integers using `let` statements or
explicitly via the `as` operator:

```rust,no_run,noplaypen
let x = Order::FIRST; // the type is Order (inference)
let y: u8 = Order::SECOND; // the type is u8 (implicit casting)
let z = Order::SECOND as u8; // the type is u8 (explicit casting)
```
