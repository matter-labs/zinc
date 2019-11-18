# Declaration

The declaration statements declare a new item.

## `let` declaration

`let [mut] {identifier}[: {type}] = {expression};`

The `let` declaration behaves just like in Rust, but it does not allow uninitialized variables.

The type is optional and is used mostly to cast integer literal or double check the expression result type, otherwise it is inferred.

Shadowing is not allowed \(already or yet\).

```rust
let mut variable: field = 0;
```

## `type` declaration

`type {identifier} = {type};`

The `type` statement declares a type alias to avoid repeating complex types.

```rust
type Alias = (field, u8, [field; 8]);
```

## `struct` declaration

This statement is a special case of `type` and declares a structure type.

```rust
struct Data {
    a: field,
    b: u8,
    c: (),
};
```

## `enum` declaration

This statement is a special case of `type` and declares an enumeration type.

```rust
enum List {
    A = 1,
    B = 2,
    C = 3,
};
```

## `fn` declaration

The `fn` statement declares a function, which is basically a callable type differing in its input arguments and return type.

```rust
fn sum(a: u8, b: u8) -> u8 {
    a + b
};
```

## `mod` declaration

`mod {identifier};`

The `mod` statement declares a new module and behaves the same way as in Rust.

## `use` import

`use {path};`

The `use` statement imports an item from another namespace and behaves the same way as in Rust.

