# Structure

No known differences from the Rust behavior.

```rust
struct Person {
    age: u8,
    id: u64,
};

let mut person = struct Person {
    age: 24,
    id: 123456789 as u64,
};
person.age = 25;
```

