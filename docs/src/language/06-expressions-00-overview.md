# Expressions

Expressions consist of operands and operators.

## Operands

- unit value
- variable
- literal
- type
- block
- conditional
- array
- tuple
- structure

All the operand types expect blocks and conditionals are simple and generally
just describe a single value in a single memory place.

Blocks and conditionals also represent a value, but they also have some side
effects. Blocks execute their statements, whereas conditionals behave in some
special way:
- both branches are always executed
- conditionals create a name scope for variables
- all assignments inside a conditinal block are implemented as conditional assignments
- heavy function calls must be optimized with a stack (to explain in detail; this is tricky because it must be applied to the nested function calls)

## Value expressions

All the expressions that represent a value. That is, anything except `type`.

## Place expressions

Expressions which describe an allocated location in memory. That is,
an identifier with optional array indexes and tuple or structure fields.

## Type expressions

These ones are used only as the second operand of the casting operator. In all
other places types are not a part of an expressions, e.g. in `struct` or `let`
statements.
