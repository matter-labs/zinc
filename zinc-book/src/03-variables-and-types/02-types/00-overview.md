# Types

Zinc is a statically typed language, thus all the variables must have a type
known at the compile-time. Strict type system allows to catch the majority of
runtime errors, which are very common to dynamically typed languages.

> If you are familiar with Rust, you will find the Zinc type system very similar,
> but with some modifications, limitations, and restrictions.

Types are divided into several groups:

- [Scalar](./01-scalar.md)
- [Array](./02-arrays.md)
- [Tuple](./03-tuples.md)
- [Structure](./04-structures.md)
- [Enumeration](./05-enumerations.md)
- [String](./06-strings.md)

To read more about casting, conversions, and type policy, go to [this chapter](./07-casting-and-conversions.md).

You can declare type aliases in Zinc, which allow you to shorten type
signatures of complex types by giving them a name:

```rust,no_run,noplaypen
type ComplexType = [(u8, [bool; 8], field); 16];

fn example(data: ComplexType) {}
```
