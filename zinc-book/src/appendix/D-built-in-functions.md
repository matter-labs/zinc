# Built-in functions

## `dbg`

Prints its arguments to the terminal. Only for debugging purposes.

Arguments:
- format string literal (`str`)
- rest of the arguments to print

Return type: `()`

**Note**: This function is special, as it accepts an arbitrary number of arguments
of any type after the format string.

## `assert`

Checks if the boolean expression is true. If it is false, the circuit fails with
an error passed as the second argument.

Arguments:
- boolean expression (`bool`)
- error message string literal (`str`)

Return type: `()`
