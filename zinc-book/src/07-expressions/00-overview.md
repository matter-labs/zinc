# Expressions

Expressions consist of operands and operators.

Operators have already been described in **Chapter 6**.

## Operands

Any syntax constructions computed into values can be used in expressions.
Zinc does all the type checking at compile-time, so you can build expressions
of arbitrary complexity without caring about safety.
However, you should care about readability and maintainability, since there are
probably other people going to work with your code.

There are several complex operands worth mentioning. Most of them except the
structure literal completely follow the Rust syntax.

## Array literal

```rust,no_run,noplaypen
let array = [1, 2, 3, 4, 5];
```

The inner type and array length are inferred by the compiler.

## Tuple literal

```rust,no_run,noplaypen
let tuple = (42, true, [1, 2, 3]);
```

The inner types and the tuple type are inferred by the compiler.

## Structure literal

There is a slight difference between Rust and Zinc structure literals.
The Zinc one starts with a `struct` keyword.

```rust,no_run,noplaypen
struct Data {
    value: field,
}

let data = struct Data {
    value: 0,
};
```

## Block

A block contains of zero or more statements and an optional result expression.
Each block has its own scope of visibility.

```rust,no_run,noplaypen
let c = {
    let a = 5;
    let b = 10;
    a + b
};
```

## Conditional

A conditional expression consists of the condition, main block, and optional
`else` block.
Each block has its own scope of visibility.

```rust,no_run,noplaypen
let condition = true;
let c = if condition {
    let a = 5;
    a
} else {
    let b = 10;
    b
};
```

## Match

A match expression is actually a syntactic sugar for nested conditional
expressions. Each branch block has its own scope of visibility.

```rust,no_run,noplaypen
let value = MyEnum::ValueOne;

match value {
    MyEnum::ValueOne => { ... }
    MyEnum::ValueTen => { ... }
    42 => { ... }
    _ => { ... }
}
```

For now, only these match patterns are supported:
- constant (e.g. `42`)
- path (e.g. `MyEnum::ValueOne`)
- variable binding (e.g. `value`)
- wildcard (`_`)
