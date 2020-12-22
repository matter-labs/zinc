# Intrinsic functions

Intrinsic functions are special and usually correspond to dedicated Zinc VM
instructions.

## `dbg`

Prints its arguments to the terminal. Only for debugging purposes.

Arguments:
- format string literal (`str`)
- rest of the arguments to print

Return type: `()`

**Note**: This function is special, as it accepts an arbitrary number of arguments
of any type after the format string.

## `require`

Checks if the boolean expression is true. If it is not, the circuit fails with
an error passed as the second argument.

Arguments:
- boolean expression (`bool`)
- error message string literal (`str`)

Return type: `()`

This is the only function able to halt the application execution.

## `<Contract>::transfer` function

Executes a transfer which is eventually sent to the zkSync platform.

Is automatically defined as a method in every smart contract.

Arguments:
- sender: `<Contract>`
- recipient: `u160`
- token_address: `u160`
- amount: `u248`

Returns: `()`
