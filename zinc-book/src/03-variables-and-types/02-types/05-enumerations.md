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

## Implementation

An enumeration can be implemented, that is, some methods and associated items
may be declared for it. The enumeration implementation resemble the behavioral
part of a class in object-oriented language.

```rust,no_run,noplaypen
enum List {
    First = 1,
    Second = 2,
    Third = 3,
}

impl List {
    pub fn first() -> Self {
        Self::First
    }

    pub fn second() -> Self {
        Self::Second
    }

    pub fn third() -> Self {
        Self::Third
    }
}

fn main(witness: field) -> field {
    (List::first() + List::second() + List::third()) as field * witness
}
```

For more information on methods, see this [chapter](../03-functions.md).

