# Function

The function is the only callable type in Zinc and it closely follows the Rust
syntax. However, R1CS specifics require that functions must be executed completely,
thus there is no `return` statement in Zinc. The only way to return a value is
to specify it as the last unterminated statement of the function block.

Functions consist of several parts: the name, arguments, return type, and the
code block. The function name uniquely defines the function within its namespace.
The arguments can be only passed by value, and the function result can only be
returned by value. If the return type is omitted, the function is considered
to return a void value `()`. The code block can access the global scope,
but it has no information about where the function has been called from.

```rust,no_run,noplaypen
const GLOBAL: u8 = 31;

fn wierd_sum(a: u8, b: u8) -> u8 {
    side_effect(); // a statement
    a + b + GLOBAL // return value
}

let result = wierd_sum(42, 27);
assert!(result == 100, "the weird sum is incorrect");
```
