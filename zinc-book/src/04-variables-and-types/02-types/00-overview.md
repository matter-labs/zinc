# Types

Zinc is a statically typed language, thus all the variables must have a type
known at the compile time. Strict type system allows to catch the majority of
runtime errors, which are very common to dynamically typed languages. Zinc type
system closely resembles that of Rust, but with some modifications, limitations,
and restrictions.

Types are divided into three major classes: scalar, compound, and custom.
Unit, boolean, integer, and string are the scalar types and you can learn more
about them in [this chapter](./01-scalar.md).
Arrays and tuples are the compound types explained in [this chapter](./02-compound.md).
Structures, enumerations, and functions are custom types described in [this chapter](./03-custom.md).
Custom types differ from others in having a user-defined name and special
declaration syntax.
The string type does not belong to any of the classes described above since
it is used mostly to annotate statements and provide error messages. For more
details, check the [this chapter](./04-other.md).

You can also declare type aliases in Zinc, which allow you to shorten type
signatures of complex types by giving them a name:

```rust,no_run,noplaypen
type ComplexType = [(u8, [bool; 8], field); 16];

fn example(data: ComplexType) {}
```

## Casting and conversions

The language enforces static strong explicit type semantics. It is the most
strict type system available since reliability is above everything. However,
some inference abilities will not do any harm, so you do not have to specify
types in places where they are highly obvious.

### Explicit

Type conversions can be only performed on the integer and enumeration types with
the casting operator. [Chapter 6](../../06-operators/00-overview.md) explains
the operator behavior in detail.

### Implicit

The `let` statement can perform implicit type casting of integers if the type
is specified to the left of the assignment symbol. Let us examine the statement:

```rust,no_run,noplaypen
let a: field = 42 as u32;
```

1. `42` is inferred as a value of type `u8`.
2. `42` is casted from `u8` to `u32`.
3. The expression `42 as u32` result is casted to `field`.
4. The field value is assigned to the variable `a`.

The second case of implicit casting is the negation operator, which always
returns a signed integer type value of the same bitlength, regardless of the
input argument.

```rust,no_run,noplaypen
let positive = 100; // u8
let negative = -positive; // i8
```

[Chapter 6](../../06-operators/00-overview.md) describes the negation operator
with more detail.

### Inference

For now, Zinc infers types in two cases: integer literals and `let` bindings.

Integer literals are always inferred as values of the minimal possible size.
That is, `255` is a `u8` value, whereas `256` is a `u16` value. Signed integers
must be implicitly casted using the negation operator.

The `let` statement can infer types in case its type is not specified.

```rust,no_run,noplaypen
let value = 0xffffffff_ffffffff_ffffffff_ffffffff;
```

In the example above, the `value` variable gets type `u128`, since 128 bytes
are enough to represent the value `0xffffffff_ffffffff_ffffffff_ffffffff`;
