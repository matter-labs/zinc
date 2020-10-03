# Smart contracts

A Zinc smart contract consists of the entry file `main.zn`, where the contract
itself is declared, and zero or more modules, whose contents can be imported
into the main file.

## Example

### Entry point file

```rust,no_run,noplaypen
/// 
/// 'src/cube_deposit.zn'
///
/// Triples the deposited amount.
///

mod simple_math;

use simple_math::cube;

contract CubeDeposit {
    pub balance: u64;

    pub fn deposit(mut self, amount: u64) {
        self.balance += cube(amount);
    }
}
```

### Module `simple_math` file

```rust,no_run,noplaypen
/// 
/// 'src/simple_math.zn'
/// 

/// Returns x^3.
fn cube(x: u64) -> u64 {
    x * x * x
}
```
