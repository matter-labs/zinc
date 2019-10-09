# Enum

Simple C-like enums are supported, following the restricted Rust syntax.

The enum values can be implicitly casted to unsigned integers of enough
bitlength in expressions and `let` statements, but this behavior will be
probably forbidden in the future.

## Examples

```jab
enum Order {
    FIRST, // 0
    SECOND, // 1
}

let x = Order::FIRST; // Order
let y: u8 = Order::SECOND; // u8
```
