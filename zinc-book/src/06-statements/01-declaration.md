# Declaration statements

The declaration statements declare a new item, that is, a type, variable or module.

## `let` variable declaration

`let [mut] {identifier}[: {type}] = {expression};`

The `let` declaration behaves just like in Rust, but it does not allow
uninitialized variables.

The type is optional and is used mostly to cast integer literal or double-check
the expression result type, otherwise, it is inferred.

```rust,no_run,noplaypen
let mut variable: field = 0;
```

## `type` alias declaration

`type {identifier} = {type};`

The `type` statement declares a type alias to avoid repeating complex types.

```rust,no_run,noplaypen
type Alias = (field, u8, [field; 8]);
```

## `struct` type declaration

The `struct` statement declares a structure.

```rust,no_run,noplaypen
struct Data {
    a: field,
    b: u8,
    c: (),
}
```

## `enum` type declaration

The `enum` statement declares an enumeration.

```rust,no_run,noplaypen
enum List {
    A = 1,
    B = 2,
    C = 3,
}
```

## `fn` type declaration

The `fn` statement declares a function.

```rust,no_run,noplaypen
fn sum(a: u8, b: u8) -> u8 {
    a + b
}
```

## `impl` namespace declaration

The `impl` statement declares a namespace of a structure or enumeration.

```rust,no_run,noplaypen
struct Data {
    value: field,
}

impl Data {
    fn print(self) {
        dbg!("{}", data.value);
    }
}
```

## `mod` module declaration

`mod {identifier};`

The `mod` statement declares a new module and requires an eponymous module file
to be present in the declaring module directory.

That is, if your declare a module named `utils` in the file `main.zn` located in
the `src/` directory, there must be a file `src/utils.zn`.

The Zinc module system almost completely mimics [that of Rust](https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html),
but requires every module to reside in a separate file and temporarily allows
importing private items.

## `use` module import

`use {path};`

The `use` statement imports an item from another namespace to the current one.

Using the example above, you may import items from your `utils` module this way:

```rust,no_run,noplaypen
mod utils;

use utils::UsefulUtility;

// some code using 'UsefulUtility'
```

## `contract` declaration

The `contract` statement declares a smart contract. Contracts are described
[here](../07-smart-contracts/00-overview.md).
The statement is a merged `struct` and `impl` statements, but it can be only
declared in the entry point file.

```rust,no_run,noplaypen
type Currency = u248;
type PairToken = u8;

contract Uniswap {
    // The contract storage fields     
    balance_1: Currency;
    balance_2: Currency;    
    rate: u248;
    
    // Public entries available from outside   
    
    pub fn deposit(self, amount: Currency, token: PairToken) {
        // ...
    }

    pub fn withdraw(self, amount: Currency) {
        // ...
    }
    
    pub fn buy(self, amount: Currency, from: PairToken) {
        // ...
    }
    
    // Private functions
    
    fn foo(self) {
        // ...
    }
}
```
