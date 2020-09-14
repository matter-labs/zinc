# Function

The function is the only callable type in Zinc. However, R1CS specifics require
that functions must be executed completely, thus there is no `return` statement.
The only way to return a value is to specify it as the last unterminated
statement of the function block.

Functions consist of several parts: the name, arguments, return type, and the
code block. The function name uniquely defines the function within its namespace.
The arguments can be only passed by value, and the function result can only be
returned by value. If the return type is omitted, the function is considered
returning a unit value `()`. The code block can access the global scope,
but it has no information about where the function has been called from.

```rust,no_run,noplaypen
const GLOBAL: u8 = 31;

fn wierd_sum(a: u8, b: u8) -> u8 {
    dbg!("{} + {}", a, b);
    a + b + GLOBAL // return value
}

fn main() {
    let result = wierd_sum(42, 27);
    assert!(result == 100, "the weird sum is incorrect");
}
```

## Methods

Methods are functions declared in a structure or enumeration implementation,
or in a smart contract definition. Such functions accept the object instance as
the first argument and can be called via the dot operator.

```rust,no_run,noplaypen
struct Data {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

impl Data {
    pub fn sum(self) -> u8 {
        self.a + self.b + self.c + self.d
    }
}

fn main() {
    let data = Data { a: 1, b: 2, c: 3, d: 4 };
    
    dbg!("Data sum is: {}", data.sum());
}
```

Methods can be called like ordinary functions using the type namespace they
are declared in. In some languages it is called a static form:

```rust,no_run,noplaypen
dbg!("Data sum is: {}", Data::sum(data));
```

## Constant functions

Constant functions are called at compile-time, thus they may only accept and
return constant expressions. Such functions are useful when you need to use
a lot of similar parameterized values, and you are not willing to repeat the
calculating code each time.

```rust,no_run,noplaypen
const fn cube(x: u64) -> u64 { x * x * x }

fn main() {
    let cubed_ten = cube(10 as u64); // 1000
    let cubed_twenty = cube(20 as u64); // 8000
}
```

Such functions only exist at compile time, so they do not impact the circuit
performance at all.
