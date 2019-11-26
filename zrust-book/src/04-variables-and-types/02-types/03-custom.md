# Custom types

Structure, enumeration, and function are the custom types. They must be
explicitly declared in the code using the `struct`, `enum`, and `fn` statements
respectively. These types always have a name allowing to distinguish them,
even if their signatures are the same. Thus, compound types facilitate type
checking and reduce code verbosity and repeatability.

## Structure

A structure is a custom data type that lets you name and package together
multiple related values that make up a meaningful group. Structures allow you
to easily build complex data types and pass them around your code with as little
verbosity as possible.

Structure fields can be accessed via the dot operator, which is explained in
detail in **Chapter 5**.

```rust,no_run,noplaypen
struct Person {
    age: u8,
    id: u64,
}

let mut person = struct Person {
    age: 24,
    id: 123456789 as u64,
};
person.age = 25;
```

## Enumeration

Enums allow you to define a type by enumerating its possible values. Only simple
C-like enums are supported for now, which are groups of constants, following
the Rust syntax:

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

The enum values can be implicitly casted to unsigned integers of enough
bitlength using `let` statements or explicitly using the `as` operator:

```rust,no_run,noplaypen
let x = Order::FIRST; // the type is Order (inference)
let y: u8 = Order::SECOND; // the type is u8 (implicit casting)
let z = Order::SECOND as u8; // the type is u8 (explicit casting)
```

## Function

The function is the only callable type in ZRust and it closely follows the Rust
syntax. However, R1CS specifics require that functions must be executed completely,
thus there is no `return` statement in ZRust. The only way to return a value is
to specify it as the last unterminated statement of the function block.

Functions consist of several parts: the name, arguments, return type, and the
code block. The name is the function type name and it uniquely defines a function.
The arguments can be only passed by value, and the function result can only be
returned by value. If the return type is omitted, the function is considered
to return the result of type `()`. The code block can access the global scope,
but it has no information about where the function has been called from.

```rust,no_run,noplaypen
const GLOBAL: u8 = 31;

fn wierd_sum(a: u8, b: u8) -> u8 {
    side_effect(); // a statement
    a + b + GLOBAL // return value
}

let result = wierd_sum(42, 27);
std::require(result == 100);
```
