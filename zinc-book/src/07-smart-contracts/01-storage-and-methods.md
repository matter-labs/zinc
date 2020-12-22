# Storage and methods

A typical contract consists of several groups of entities:

- implicit storage fields
- explicit storage fields
- the constructor
- public methods
- private methods
- built-in methods
- global variables
- constants

## Implicit storage fields

There are several implicitly created fields, which are set upon the contract publishing:

- the contract address (field `address` of type `u160`)
- the contract balances (field `balances` of type `std::collections::MTreeMap<u160, u248>`),
where the key is a zkSync token address, and the value is token amount.

So, when you see an empty contract `contract Empty {}`, it actually looks like this:

```rust,no_run,noplaypen
// will not compile, because the fields are already there!
contract Empty {
    pub address: u160;

    pub balances: std::collections::MTreeMap<u160, u248>;
}
```

The public (`pub`) fields are visible when querying the contract storage state,
whereas the private fields are internal and cannot be seen.

## Explicit storage fields

The explicit storage fields are declared in the same way as in structure, but with
a semicolon in the end.

```rust,no_run,noplaypen
contract Example {
    pub tokens: (u8, u64);

    data: [u8; 1000];

    //...
}
```

Each smart contract instance gets its own storage, which is written to the
persistent databases by the Zinc Zandbox server.

## The constructor

Each contract must have a constructor, a special function with the name `new`, which
returns a `Self` contract instance. The contract instance is not a value, but a reference
to it, that is, its ETH address of type `u160`.

You must not initialize the implicit storage fields, since they are filled automatically.

```rust,no_run,noplaypen
contract Example {
    pub value: u64;

    pub fn new(_value: u64) -> Self {
        Self {
            value: _value,
        }
    }
}
```

## Public methods

The contract declaration contains several public functions, which serve as
contract methods. The contract must have at least one public function.

```rust,no_run,noplaypen
contract Example {
    //...

    pub fn deposit(mut self, amount: u64) -> bool { ... }
}
```

## Private methods

The private functions are declared without the `pub` keyword and have no
special meaning. Such functions are simply associated with the contract and
can be called from within the public methods.

```rust,no_run,noplaypen
contract Example {
    //...

    fn get_balance(address: u160) -> bool { ... }
}
```

## Builtin methods

Each smart contract includes the built-in `transfer` method, which is used to send
tokens to another account. The method is mutable, so it can only be called from
the mutable context.

```rust,no_run,noplaypen
contract Example {
    //...

    fn send(mut self, address: u160, amount: u248) -> {
        self.transfer(
            address, // recipient address
            0x0, // zkSync ETH token address
            amount, // amount in wei
        );
    }
}
```

The method signature in described in [Appendix D](../appendix/D-intrinsic-functions.md).

## Global variables

Each contract includes the global `zksync::msg` variable, which contains the
transfer data the contract has been called with. The variable description can
be found in the [Appendix F](../appendix/F-zksync-library.md).

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
