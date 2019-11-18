# Expressions

Expressions consist of operands and operators.

## Operands

* unit value
* variable
* literal
* type
* block
* conditional
* array
* tuple
* structure

All the operand types expect blocks and conditionals are simple and generally just describe a single value in a single memory place.

Blocks and conditionals also represent a value, but they also have some side effects. Blocks execute their statements, whereas conditionals behave in some special way:

* both branches are always executed
* conditionals create a name scope for variables
* assignments in a conditinal block are implemented as conditional assignments
* heavy function calls must be optimized with a stack \(this is tricky because

  it must be applied to the nested function calls\)

