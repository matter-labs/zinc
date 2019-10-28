# Statements

The following statements have been implemented so far:

- empty statement
- `let` declaration
- `for` loop
- `type` declaration
- `struct` declaration
- `enum` declaration
- `fn` declaration
- `mod` declaration
- `use`
- expression statement

An important difference from Rust: all statements must be terminated with
a `;`. That is, all statements including `struct` declaration and expression
ones ending with `}` must be terminated with `;` as well.

## Empty

An empty statement is just a `;`.

## Let declaration

The `let` declaration behaves just like in Rust, but it does not allow
uninitialized variables.

Shadowing is not allowed yet.

### Example

```rust
let mut variable: field = 0;
```

## Loop

The `for` loop statement behaves just like in Rust, but it is merged with the
`while` loop, so the optional `while` condition is checked before each iteration
of the loop. The `while` condition expression has access to the inner scope and
can use its variables and the loop iterator.

### Examples

```rust
    for i in 0..=10 while i % x != 8 {
        debug(i);
    }
```

## Type

The `type` statement declares a type alias to avoid repeating complex types.

### Example

```rust
type Alias = (field, u8, [field; 8]);
```

## Struct

The `struct` statement declares a structure type.

### Example

```rust
struct Data = {
    a: field,
    b: u8,
    c: (),
};
```
