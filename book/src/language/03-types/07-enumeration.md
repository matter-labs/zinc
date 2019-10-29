# Enumeration

Simple C-like enums are supported, following the restricted Rust syntax:

```rust
enum Order {
    FIRST = 0,
    SECOND = 1,
};
```

The enum values can be implicitly casted to unsigned integers of enough
bitlength in `let` statements:

```rust
let x = Order::FIRST; // the type is Order
let y: u8 = Order::SECOND; // the type is u8
```
