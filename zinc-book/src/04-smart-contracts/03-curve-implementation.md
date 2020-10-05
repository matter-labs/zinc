# The Curve

The [Curve smart contract](https://www.curve.fi/stableswap-paper.pdf) has been
partially ported from its original
[Vyper implementation](https://github.com/curvefi/curve-contract/blob/2b8ff42f5ce648be749c721d23c28ec8483df493/vyper/stableswap.vy).

The full Zinc source code is [here](https://github.com/matter-labs/curve-zinc).

## Code listings

Here are the most important parts of the Curve implementation. Some boilerplate
code with types and constants is omitted and can be checked out via the link above.

#### Main module

```rust,no_run,noplaypen
//!
//! The Curve Stableswap contract.
//!

mod types;
mod invariant;
mod constants;
mod exchange;

use self::constants::ZERO;
use self::constants::N;
use self::types::Address;
use self::types::Balance;
use self::types::token_id::TokenId;
use self::types::transaction::Transaction;

///
/// The Curve Stableswap contract.
///
contract Stableswap {
    /// The contract address.
    pub extern address: Address;

    /// The tokens being traded in the pool.
    pub tokens: [TokenId; N];

    /// The contract balances.
    pub balances: [Balance; N];

    /// The Curve amplifier.
    pub amplifier: u64;

    ///
    /// The contract constructor.
    ///
    pub fn new(
        _tokens: [TokenId; N],
        _amplifier: u64
    ) -> Self {
        assert!(_amplifier > 0, "The Curve amplifier cannot be zero");

        Self {
            address: 0x0 as Address,
            tokens: _tokens,
            balances: [ZERO; N],
            amplifier: _amplifier,
        }
    }

    ///
    /// Adds liquidity to the contract balances.
    ///
    pub fn deposit(
        mut self,
        tx: Transaction,
    ) {
        assert!(tx.recipient == self.address, "Transaction recipient is not the contract");

        let deposit_idx = self.token_position(tx.token_id);
            
        self.balances[deposit_idx] += tx.amount;
    }

    ///
    /// Exchanges the tokens, consuming some of the `tx.token_id` and returning some of the
    /// `withdraw_token_id` to the client.
    ///
    pub fn swap(
        mut self,
        tx: Transaction,
        withdraw_address: Address,
        withdraw_token_id: TokenId,
        min_withdraw: Balance,
    ) {
        assert!(tx.recipient == self.address, "Transaction recipient is not the contract");
        assert!(tx.amount > 0, "Sent amount cannot be zero");
        assert!(min_withdraw > 0, "Minimal withdrawal cannot be zero");

        let deposit_idx = self.token_position(tx.token_id);
        let withdraw_idx = self.token_position(withdraw_token_id);

        assert!(self.balances[deposit_idx] != 0, "Deposit token balance is zero");
        assert!(self.balances[withdraw_idx] != 0, "Withdraw token balance is zero");

        let old_y = self.balances[withdraw_idx];

        let new_x = self.balances[deposit_idx] + tx.amount;
        let new_y = exchange::after(
            self.tokens,
            self.balances,
            self.amplifier,

            deposit_idx,
            withdraw_idx,
            new_x
        );

        assert!(old_y >= min_withdraw + new_y, "Exchange resulted in fewer coins than expected");
        let withdraw_amount = old_y - new_y;

        self.balances[deposit_idx] = new_x;
        self.balances[withdraw_idx] = new_y;

        std::zksync::transfer(withdraw_address, withdraw_token_id, withdraw_amount);
    }

    ///
    /// Given the amount to withdraw, returns the amount that must be deposited.
    ///
    pub fn get_dx(
        self,
        deposit_token_id: TokenId,
        withdraw_token_id: TokenId,
        to_withdraw: Balance,
    ) -> Balance {
        let deposit_idx = self.token_position(deposit_token_id);
        let withdraw_idx = self.token_position(withdraw_token_id);

        assert!(self.balances[deposit_idx] != 0, "Deposit token balance is zero");
        assert!(self.balances[withdraw_idx] != 0, "Withdraw token balance is zero");

        let after_withdrawal = self.balances[withdraw_idx] - to_withdraw;
        
        let after_deposit = exchange::after(
            self.tokens,
            self.balances,
            self.amplifier,

            withdraw_idx,
            deposit_idx,
            after_withdrawal,
        );

        after_deposit - self.balances[deposit_idx]
    }

    ///
    /// Given the amount to deposit, returns the amount that will be withdrawn.
    ///
    pub fn get_dy(
        self,
        deposit_token_id: TokenId,
        withdraw_token_id: TokenId,
        to_deposit: Balance,
    ) -> Balance {
        let deposit_idx = self.token_position(deposit_token_id);
        let withdraw_idx = self.token_position(withdraw_token_id);

        assert!(self.balances[deposit_idx] != 0, "Deposit token balance is zero");
        assert!(self.balances[withdraw_idx] != 0, "Withdraw token balance is zero");

        let after_deposit = self.balances[deposit_idx] + to_deposit;
        
        let after_withdrawal = exchange::after(
            self.tokens,
            self.balances,
            self.amplifier,

            deposit_idx,
            withdraw_idx,
            after_deposit,
        );

        self.balances[withdraw_idx] - after_withdrawal
    }

    /// 
    /// Given a token ID, returns the token position in the array of balances.
    /// 
    fn token_position(self, token_id: TokenId) -> u8 {
        let mut position = N;
        let mut found = false;

        for i in 0..N while !found {
            if self.tokens[i] == token_id {
                position = i;
                found = true;
            }
        }

        assert!(found, "Token ID is unknown");

        position
    }
}
```

#### The invariant module

```rust,no_run,noplaypen
//!
//! The invariant calculation.
//!

use crate::types::token_id::TokenId;
use crate::constants::ZERO;
use crate::constants::N;

///
/// The `D` invariant calculation function.
///
/// The function is quite generic and does not work on token balances directly.
/// The only requirement for the `values` is to be of the same precision to avoid incorrect amplification.
///
pub fn calculate(
    values: [u248; N],
    amplifier: u64,
) -> u248 {
    let mut sum = ZERO;
    for i in 0..N {
        sum += values[i];
    }

    if sum != ZERO {
        let mut D_prev = ZERO;
        let mut D = sum;

        let amplifier_N: u248 = amplifier * (N as u64);

        for _n in 0..15 while (D > D_prev && D - D_prev > 0) || (D <= D_prev && D_prev - D > ZERO) {
            let mut D_P = D;

            for i in 0..N {
                D_P = D_P * D / (values[i] * (N as u248) + 1); // +1 is to prevent division by 0
            }

            D_prev = D;
            D = (amplifier_N * sum + D_P * (N as u248)) * D / ((amplifier_N - 1) * D + ((N + 1) as u248) * D_P);
        }

        D
    } else {
        ZERO
    }
}
```

#### The swap module

```rust,no_run,noplaypen
//!
//! The swap consequences calculation.
//!

use crate::types::Balance;
use crate::types::token_id::TokenId;
use crate::constants::ZERO;
use crate::constants::PRECISION;
use crate::constants::N;

///
/// The token being withdrawn balance after the swap.
///
pub fn after(
    tokens: [TokenId; N],
    balances: [Balance; N],
    amplifier: u64,

    token_x_idx: u8,
    token_y_idx: u8,
    after_y: Balance,
) -> Balance {
    assert!(token_x_idx != token_y_idx, "Cannot exchange between the same coins");

    let mut balances_p = balances;
    for i in 0..N {
        balances_p[i] *= tokens[i].magnitude_diff() * PRECISION;
    }

    let D = crate::invariant::calculate(balances_p, amplifier);
    let An: Balance = amplifier * (N as u64);

    let mut c = D;
    let mut S: Balance = ZERO;

    for i in 0..N {
        if i == token_x_idx as u8 {
            let after_y_p = after_y * tokens[token_x_idx].magnitude_diff() * PRECISION;
            S += after_y_p;
            c = c * D / (after_y_p * (N as Balance));
        } else if i != token_y_idx as u8 {
            S += balances_p[i];
            c = c * D / (balances_p[i] * (N as Balance));
        };
    }

    c = c * D / (An * (N as Balance));
    let b: Balance = S + D / An;

    let mut y_prev: Balance = ZERO;
    let mut y: Balance = D;
    for n in 0..15 while (y > y_prev && y - y_prev > ZERO) || (y <= y_prev && y_prev - y > ZERO) {
        y_prev = y;
        y = (y * y + c) / (2 * y + b - D);
    }

    y / (tokens[token_y_idx].magnitude_diff() * PRECISION)
}
```
