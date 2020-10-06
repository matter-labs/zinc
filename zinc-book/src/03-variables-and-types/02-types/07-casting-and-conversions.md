# Casting and conversions

The language enforces static strong explicit type semantics. It is the strictest
type system available since reliability is above everything. However,
some inference abilities will not do any harm, so you do not have to specify
types in places where they are highly obvious.

## Explicit

Type conversions can be only performed on the integer and enumeration types with
the casting operator. [This chapter](../../04-operators/05-casting.md) explains
the operator's behavior in detail.

## Implicit

The `let` statement can perform implicit type casting of integers if the type
is specified to the left of the assignment symbol. Let us examine the statement:

```rust,no_run,noplaypen
let a: field = 42 as u32;
```

1. `42` is inferred as a value of type `u8`.
2. `42` is cast from `u8` to `u32`.
3. The expression `42 as u32` result is cast to `field`.
4. The field value is assigned to the variable `a`.

The second case of implicit casting is the negation operator, which always
returns a signed integer type value of the same bitlength, regardless of the
input argument.

```rust,no_run,noplaypen
let positive = 100; // u8
let negative = -positive; // i8
```

[This chapter](../../04-operators/01-arithmetic.md) describes the negation operator
in more detail.

## Inference

For now, Zinc infers types in two cases: integer literals and `let` bindings.

Integer literals are always inferred as values of the minimal possible size.
That is, `255` is a `u8` value, whereas `256` is a `u16` value.

The `let` statement can infer types in case its type is not specified.

```rust,no_run,noplaypen
let value = 0xffffffff_ffffffff_ffffffff_ffffffff;
```

In the example above, the `value` variable gets type `u128`, since 128 bytes
are enough to represent the value `0xffffffff_ffffffff_ffffffff_ffffffff`;
