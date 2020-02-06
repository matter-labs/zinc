# Literals

Simple literal operands are the basic elements of an expression:

- `42` - integer
- `false` - boolean
- `"error"` - string
- `u128` - type (in casting clauses like `42 as u128`)

There are several complex operands worth mentioning. As you will see from the
examples, you can nest these constructions as much as you need, but do not abuse
this ability too much.

## Array

```rust,no_run,noplaypen
let array = [
    1,
    2,
    3,
    4,
    5,
    1 + 5,
    { let t = 5; t * t },
];
```

The inner type and array length are inferred by the compiler.

## Tuple

```rust,no_run,noplaypen
let tuple = (42, true, [1, 2, 3]);
```

The inner types and the tuple type are inferred by the compiler.

## Structure

The Zinc one starts with a `struct` keyword.

```rust,no_run,noplaypen
struct Data {
    value: field,
}

let data = Data {
    value: 0,
};
```
