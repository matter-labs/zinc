# Maps

The `std::collections::MTreeMap` is a special type, which can only be used as a smart contract
storage field:

```rust,no_run,noplaypen
use std::collections::MTreeMap;

contract Test {
    data: MTreeMap<u8, bool>;

    pub fn example(mut self) {
        let (old1, existed1) = self.data.insert(42, true);
        let (value, exists1) = self.data.get(42);
        let exists2 = self.data.contains(42);
        let (old2, existed2) = self.data.remove(42);
    }
}
```

> The maps introduce a new concept of generic types, but this feature can
> only be used to specify the key and value types for the `MTreeMap` instance.
