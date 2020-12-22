# Minimal example

In this example we will implement the simplest exchange smart contract, where
it will be possible to exchange between a pair of tokens with ever-constant price.

You will need `zargo`, which is the Zinc package manager, which bundles smart
contract projects and simplifies usage of contract methods, using input data
JSON template located in the project `data` directory.

## Project initialization

To create a new smart contract project, use the following command:

```bash,no_run,noplaypen
zargo new --type contract constant_price
```

Zargo will create a project with some default template code:

```rust,no_run,noplaypen
//!
//! The 'constant_price' contract entry.
//!

contract ConstantPrice {
    pub value: u64;

    pub fn new(value: u64) -> Self {
        Self {
            value: value,
        }
    }
}
```

Let's change the code by:

- removing the redundant `value` auto-generated field
- adding the `fee` parameter 
- adding two mutable methods for making `exchange`s and `deposit`s
- adding an immutable method for getting the contract fee value
- declaring the `Address` and `Balance` type aliases
- declaring the `TokenAddress` enumeration type with zkSync token addresses

The `TokenAddress` enumeration lists the token address-like identifiers from
the `Rinkeby` zkSync network. Addresses of tokens on the `Rinkeby` network
should not change and may be taken from here for further usage.

```rust,no_run,noplaypen
type Address = u160;
type Balance = u248;

enum TokenAddress {
    ETH = 0x0000000000000000000000000000000000000000,
    USDT = 0x3b00ef435fa4fcff5c209a37d1f3dcff37c705ad,
    USDC = 0xeb8f08a975ab53e34d8a0330e0d34de942c95926,
    LINK = 0x4da8d0795830f75be471f072a034d42c369b5d0a,
    TUSD = 0xd2255612f9b045e9c81244bb874abb413ca139a3,
    HT = 0x14700cae8b2943bad34c70bb76ae27ecf5bc5013,
    OMG = 0x2b203de02ad6109521e09985b3af9b8c62541cd6,
    TRB = 0x2655f3a9eeb7f960be83098457144813ffad07a4,
    ZRX = 0xdb7f2b9f6a0cb35fe5d236e5ed871d3ad4184290,
    BAT = 0xd2084ea2ae4bbe1424e4fe3cde25b713632fb988,
    REP = 0x9cac8508b9ff26501439590a24893d80e7e84d21,
    STORJ = 0x8098165d982765097e4aa17138816e5b95f9fdb5,
    NEXO = 0x02d01f0835b7fdfa5d801a8f5f74c37f2bb1ae6a,
    MCO = 0xd93addb2921b8061b697c2ab055979bbefe2b7ac,
    KNC = 0x290eba6ec56ecc9ff81c72e8eccc77d2c2bf63eb,
    LAMB = 0x9ecec4d48efdd96ae377af3ab868f99de865cff8,
    GNT = 0xd94e3dc39d4cad1dad634e7eb585a57a19dc7efe,
    MLTT = 0x690f4886c6911d81beb8130db30c825c27281f22,
    XEM = 0xc3904a7c3a95bc265066bb5bfc4d6664b2174774,
    DAI = 0x2e055eee18284513b993db7568a592679ab13188,
}

impl TokenAddress {
    pub fn is_known(address: Address) -> bool {
        match address {
            0x0000000000000000000000000000000000000000 => true,
            0x3b00ef435fa4fcff5c209a37d1f3dcff37c705ad => true,
            0xeb8f08a975ab53e34d8a0330e0d34de942c95926 => true,
            0x4da8d0795830f75be471f072a034d42c369b5d0a => true,
            0xd2255612f9b045e9c81244bb874abb413ca139a3 => true,
            0x14700cae8b2943bad34c70bb76ae27ecf5bc5013 => true,
            0x2b203de02ad6109521e09985b3af9b8c62541cd6 => true,
            0x2655f3a9eeb7f960be83098457144813ffad07a4 => true,
            0xdb7f2b9f6a0cb35fe5d236e5ed871d3ad4184290 => true,
            0xd2084ea2ae4bbe1424e4fe3cde25b713632fb988 => true,
            0x9cac8508b9ff26501439590a24893d80e7e84d21 => true,
            0x8098165d982765097e4aa17138816e5b95f9fdb5 => true,
            0x02d01f0835b7fdfa5d801a8f5f74c37f2bb1ae6a => true,
            0xd93addb2921b8061b697c2ab055979bbefe2b7ac => true,
            0x290eba6ec56ecc9ff81c72e8eccc77d2c2bf63eb => true,
            0x9ecec4d48efdd96ae377af3ab868f99de865cff8 => true,
            0xd94e3dc39d4cad1dad634e7eb585a57a19dc7efe => true,
            0x690f4886c6911d81beb8130db30c825c27281f22 => true,
            0xc3904a7c3a95bc265066bb5bfc4d6664b2174774 => true,
            0x2e055eee18284513b993db7568a592679ab13188 => true,
            _ => false,
        }
    }
}

contract ConstantPrice {
    const MAX_FEE: u16 = 10000;
    const PRECISION_MUL: Balance = 1E3;

    pub fee: u16;

    pub fn new(_fee: u16) -> Self {
        require(_fee <= Self::MAX_FEE, "The fee value must be between 0 and 10000");

        Self {
            fee: _fee,
        }
    }

    pub fn deposit(mut self) {
        // check if the transaction recipient is the contract address
        require(zksync::msg.recipient == self.address, "The transfer recipient is not the contract");

        // check if the deposited token is known to the contract
        require(TokenAddress::is_known(zksync::msg.token_address), "The deposited token is unknown");

        // check if the deposited amount is not zero
        require(zksync::msg.amount > 0, "Cannot deposit zero tokens");
    }

    pub fn exchange(
        mut self,
        withdraw_token: Address,
    ) {
        // check if the transaction recipient is the contract address
        require(zksync::msg.recipient == self.address, "The transfer recipient is not the contract");

        // check if the deposited token is known to the contract
        require(TokenAddress::is_known(zksync::msg.token_address), "The deposited token is unknown");

        // check if the withdrawn token is known to the contract
        require(TokenAddress::is_known(withdraw_token), "The withdrawn token is unknown");

        // check if the deposited amount is not zero
        require(zksync::msg.amount > 0, "Cannot deposit zero tokens");

        // check if the deposited and withdrawn token identifiers are different
        require(zksync::msg.token_address != withdraw_token, "Cannot withdraw the same token");

        let withdraw_token_amount = zksync::msg.amount *
            ((Self::MAX_FEE - self.fee) as Balance * Self::PRECISION_MUL / Self::MAX_FEE as Balance) /
            Self::PRECISION_MUL;
        // check if there is enough balance to withdraw
        require(self.balances.get(withdraw_token).0 >= withdraw_token_amount, "Not enough tokens to withdraw");

        self.transfer(zksync::msg.sender, withdraw_token, withdraw_token_amount);
    }

    pub fn get_fee(self) -> u16 {
        self.fee
    }
}
```

> In our case, the `fee` is an integer value between `0` and `10000`, where the latter
> represents `100%`. It is common practice to use integer values in this way, since there
> are usually limited support of floating point numbers in safe smart contract languages.
> We are also using some additional fractional digits to avoid getting zeros after
> integer division. That is, instead of doing `amount * 9900 / 10000`, we do
> `amount * 9900 * 1E3 / 10000 / 1E3`.

## Publishing the contract

Before publishing, run `zargo build` in the project directory. Then,
open the `./data/input.json` constructor input template file
and fill the constructor arguments you are going to pass:

```json
{
  "arguments": {
    "new": {
      "_fee": "100"
    }
    // ...
  }
}
```

> Also, put your account private key to the `./data/private_key` file. All deposits
> and transfers to the newly created contract will be done from that account.
> Ensure that your account is unlocked and has enough balance to pay fees.
> To see how to unlock a new zkSync account, go to the [troubleshooting](./04-troubleshooting.md) chapter.

To publish the contract, use this simple command with the network identifier
and instance name:

```bash,no_run,noplaypen
zargo publish --network rinkeby --instance default
```

> Since every follower of this tutorial has created a contract with name `constant_price`,
> the contract may not be uploaded, because the name and version must be unique.
> To fix this issue, you may change your contract name or version in the `Zargo.toml` manifest.
> To see all uploaded projects, use the `zargo download --list` command.

When the contract is successfully published, its ETH address and zkSync account ID
will be returned. You will need the address to make some further calls.
Let's assume it is `0x1234...1234`.

The instance name is used to uniquely identify your published contract without
memorizing its ETH address.

The contract has been published!

## Querying the contract storage

The `constant_price` contract is now published, and its dedicated storage
instance is created. You may query the Zandbox server to see its zero balances:

```bash,no_run,noplaypen
zargo query --network rinkeby --address 0x1234...1234
```

## Calling a non-mutable contract method

A non-mutable contract method can be called with the same query as above, but
with the `method` argument:

```bash,no_run,noplaypen
zargo query --network rinkeby --address 0x1234...1234 --method get_fee
```

The output:
```json
{
  "output": "100"
}
```

## Calling a mutable contract method

Let's now call our contract `deposit` method!

Open the method input template file `./data/input.json` and specify
the token identifier and amount you want to exchange:

```json
{
  "msg": {
    "sender": "<your_address>",
    "recipient": "0x1234...1234",
    "token_address": "0x0000000000000000000000000000000000000000", // ETH
    "amount": "0.1_E18"
  }
}
```

> Be cautious when specifying the exponent value for token amounts, as it is
> crucial to specify the correct number of decimal digits for each token.

To call the contract method, use the following command with the method name and
contract account ID:

```bash,no_run,noplaypen
zargo call --network rinkeby --address 0x1234...1234 --method deposit
```

After the call has succeeded, query the contract storage again to see the
expected result:

```json
{
  "address": "0x1234...1234",
  "balances": [
    {
      "key": "0x0",
      "value": "100000000000000000" // 0.1_E18
    }
  ]
}
```

Now you may repeat the call for other tokens and when there is more than one token
on the exchange, call the `exchange` method, specifying the token you want to withdraw.

## What's next

When you have a new smart contract version, just publish it with another instance
name and it will get a separate storage instance, living its own life!

Also, there is a [Curve smart contract](./03-curve-implementation.md) implementation
in Zinc. Check it out!
