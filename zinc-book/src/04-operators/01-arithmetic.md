# Arithmetic operators

Arithmetic operators do not perform any kind of overflow checking at
compile-time. If an overflow happens, the Zinc VM will fail at runtime.

> When it comes to the division of negative numbers, Zinc follows the Euclidean
> division concept. It means that `-45 % 7 == 4`. To get the detailed explanation
> and some examples, see the [article](https://en.wikipedia.org/wiki/Euclidean_division).

The `+=`, `-=`, `*=`, `/=`, `%=` shortcut operators perform the operation
and assign the result to the first operand. The first operand must be a mutable memory location
like a variable, array element, or structure field.

### Addition

`+` and `+=` are binary operators.

*Accepts*
1. Integer expression
2. Expression of the operand 1 type

*Returns* an integer result of the same type.

### Subtraction

`-` and `-=` are binary operators.

*Accepts*
1. Integer expression
2. Expression of the operand 1 type

*Returns* an integer result of the same type.

### Multiplication

`*` and `*=` are binary operators.

*Accepts*
1. Integer expression
2. Expression of the operand 1 type

*Returns* an integer result of the same type.

### Division

`/` and `/=` are binary operators.

*Accepts*
1. Integer expression (any type except `field`)
2. Expression of the operand 1 type

*Returns* an integer result of the same type.

### Remainder

`%` and `%=` are binary operators.

*Accepts*
1. Integer expression (any type except `field`)
2. Expression of the operand 1 type

*Returns* an integer result of the same type.

### Negation

`-` is an unary operator.

*Accepts*
1. Unsigned integer expression

*Returns* an integer result of the same type.
