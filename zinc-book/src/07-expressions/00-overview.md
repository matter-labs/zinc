# Expressions

Expressions consist of operands and operators.

Operators have already been described in [this chapter](../06-operators/00-overview.md).

Zinc supports constant expressions with arrays, tuples, structures, conditionals,
and matches, which are described [here](04-constant.md).

## Operands

Any syntax constructions computed into values can be used in expressions.
Zinc does all the type checking at compile-time, so you can build expressions
of arbitrary complexity without caring about type safety.
However, you should care about readability and maintainability, since there are
probably other people going to work with your code.
