# Minimal example

In this example we will implement the simplest exchange smart contract, where
it will be possible to exchange between a pair of tokens with ever-constant price.

This example will not use any means of transferring tokens between user accounts,
since its point only to demonstrate the basics of working with Zinc smart
contracts.

You will need two binaries from the Zinc framework:
- `zargo`
- `zandbox`

Zandbox is the Zinc sandbox server, which is a middleware between the user and
[zkSync](https://zksync.io/) platform. The server accepts the smart contract
source code and method call requests.

Zargo is the Zinc project manager, which bundles smart contract projects and
simplifies usage of contract methods, using input data JSON templates located
in the project data directory.

## Running the Zandbox server

To be able to publish and call smart contract locally, open another terminal
window and run the Zandbox server:

```bash
zandbox -v
```

## Project initialization

To create a new smart contract project, use the following command:

```bash
zargo new --type contract constant_price
```

Zargo will create a project with some default template code:

```rust,no_run,noplaypen
//!
//! The 'constant_price' contract entry.
//!

contract ConstantPrice {
    balance: u248;

    pub fn new(_balance: u248) -> Self {
        Self {
            balance: _balance,
        }
    }
}
```

Let's change the code to have two balance fields, the `exchange` method, and
two methods `get_x` and `get_y` to query the contract balances separately.

```rust,no_run,noplaypen
enum Token {
    X = 1,
    Y = 2,
}

type Balance = u248;

contract ConstantPrice {
    balance_x: Balance;
    balance_y: Balance;

    pub fn new(
        _balance_x: Balance,
        _balance_y: Balance,
    ) -> Self {
        Self {
            balance_x: _balance_x,
            balance_y: _balance_y,
        }
    }

    pub fn exchange(
        mut self,
        token: Token,
        amount: Balance,
    ) {
        match token {
            Token::X => {
                assert!(self.balance_y >= amount, "Not enough Y tokens to withdraw");
                self.balance_x += amount;
                self.balance_y -= amount;

                // transfer amount of X from the user
                // transfer amount of Y to the user
            },
            Token::Y => {
                assert!(self.balance_x >= amount, "Not enough X tokens to withdraw");
                self.balance_y += amount;
                self.balance_x -= amount;

                // transfer amount of Y from the user
                // transfer amount of X to the user
            },
        };
    }

    pub fn get_x(self) -> Balance {
        self.balance_x
    }

    pub fn get_y(self) -> Balance {
        self.balance_y
    }
}
```

In the listing above we declared the `Balance` type alias and the `Token`
enumeration type, which identifies the token deposited by user.

The exchange function increases and decreases the contract balances depending
on the token identifier, which can be either X or Y. When the balances are
changed, some code is called to transfer the tokens to and from user accounts.

## Publishing the contract

Before publishing, open the `./data/witness_new` constructor input template file
and fill the values you are going to pass:

```json
{
  "_balance_x": "100",
  "_balance_y": "100"
}
```

To publish the contract, use this simple command with the network identifier
and instance name:

```bash
zargo publish --network ropsten --instance default
```

To see the available testnets, enter `zargo publish --help`. When the contract
is successfully published, its ETH address and zkSync account ID will be returned.
You will need the address to make the consequent calls. Let's assume it equal to
`0x1234123412341234123412341234123412341234`.

The instance name is used to uniquely identify your published contract without
memorizing its ETH address.

The contract has been published!

## Querying the contract storage

The `constant_price` contract is now published, and its dedicated storage
instance is created. You may query the Zandbox server to see the current balances:

```bash
zargo query --network ropsten --address 0x1234123412341234123412341234123412341234
```

## Calling a non-mutable contract method

A non-mutable contract method may be called with the same query as above, but
with the `method` argument:

```bash
zargo query --network ropsten --address 0x1234123412341234123412341234123412341234 --method get_x
```

The output:
```json
{
  "output": "50"
}
```

## Calling a mutable contract method

Let's now call our contract `exchange` method!

Open the method input template file `./data/witness_exchange.json` and specify
the token identifier and amount you want to exchange:

```json
{
  "token": "Y",
  "amount": "50"
}
```

To call the contract method, use the following command with the method name and
contract account ID:

```bash
zargo call --network localhost --address 0x1234123412341234123412341234123412341234 --method exchange
```

After the call has succeeded, query the contract storage again to see the
expected result:

```json
{
  "output": {
    "balance_x": "50",
    "balance_y": "150"
  }
}
```

Congratulations! You have implemented and published a working smart contract!

## What's next

You may play with the contract a little more, e.g. by adding some exchange fee.

When you have a new smart contract version, just publish it under another ID and
it will get a separate storage instance, living its own life!
