# Structures

The structure is a custom data type that lets you name and package together
multiple related values that make up a meaningful group. Structures allow you
to easily build complex data types and pass them around your code with as little
verbosity as possible.

Structure fields can be accessed via the dot operator, which is explained in
detail [here](../../05-operators/00-overview.md).

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
