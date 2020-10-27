# The zkSync library

The zkSync library contains functions and utilities to perform operations in
the zkSync networks.

## `zksync::transfer` function

Executes a transfer which is eventually sent to the zkSync platform.

Arguments:
- recipient: `u160`
- token_id: `u16`
- amount: `u248`

Returns: `()`

## `zksync::msg` variable

The built-in global transaction variable.

Fields:
- sender: `u160`
- recipient: `u160`
- token_address: `u160`
- amount: `u248`
