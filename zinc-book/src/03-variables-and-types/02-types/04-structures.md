# Structures

The structure is a custom data type which lets you name and package together
multiple related values that make up a meaningful group. Structures allow you
to easily build complex data types and pass them around your code with as little
verbosity as possible.

Structure fields can be accessed via the dot operator, which is explained in
detail [here](../../04-operators/06-access.md).

```rust,no_run,noplaypen
struct Person {
    age: u8,
    id: u64,
}

fn main() {
    let mut person = Person {
        age: 24,
        id: 123456789 as u64,
    };
    person.age = 25;
}
```

## Implementation

A structure can be implemented, that is, some methods and associated items
may be declared for it. The structure implementation resemble the behavioral
part of a class in object-oriented language.

```rust,no_run,noplaypen
struct Arithmetic {
    a: field,
    b: field,
}

impl Arithmetic {
    pub fn add(self) -> field {
        self.a + self.b
    }

    pub fn sub(self) -> field {
        self.a - self.b
    }

    pub fn mul(self) -> field {
        self.a * self.b
    }

    pub fn div(self) {
        require(false, "Field division is forbidden!");
    }
}

fn main() {
    let a: field = 10;
    let b: field = 5;
    let arithmetic = Arithmetic { a: a, b: b };
    
    dbg!("{} + {} = {}", a, b, arithmetic.add());
    dbg!("{} - {} = {}", a, b, arithmetic.sub());
    dbg!("{} * {} = {}", a, b, arithmetic.mul());
    dbg!("{} / {} = {}", a, b, arithmetic.div()); // will panic
}
```

For more information on methods, see this [chapter](../03-functions.md).
