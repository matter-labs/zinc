# Data and functions

A typical contract consists of several groups of entities:

- storage fields
- public entry functions
- private functions
- constants

## Storage fields

The storage fields are declared in the same way as in structures.

```rust,no_run,noplaypen
contract Example {
    data: [u8; 1000],
    tokens: (u8, u64),
    address: u160,

    //...
}
```

## Public entry functions

The contract declaration contains several public functions, which serve as the
entry points of the contract. The contract must have at least one public function.

```rust,no_run,noplaypen
contract Example {
    //...

    pub fn deposit(amount: u64) -> bool { ... }
}
```

## Private functions

The private functions are declared without the `pub` keyword and they have no
special meaning. Such functions are simply associated with the contract and
can be called from within the public ones.

```rust,no_run,noplaypen
contract Example {
    //...

    fn is_valid(address: u160) -> bool { ... }
}
```

## Constants

A contract may contain some constants associated with it. The constants do not
have any special meaning and can be used from within the contract functions or
from the outside.

```rust,no_run,noplaypen
contract Example {
    //...

    pub const VERSION: u8 = 1; // public constant 

    const LIMIT: u8 = 255; // private constant
}
```
