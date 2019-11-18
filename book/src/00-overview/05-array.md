# Array

Fixed-sized arrays follow the Rust rules.

The only exception is the temporary restriction to constant indexes, that is, you cannot index an array with anything but an integer literal for now.

Arrays support the index operator:

```rust
let element = fibbonaci[3];
fibbonaci[2] = 1;
```

