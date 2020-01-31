# Arrays

Arrays are collections of values of the same type sequentially stored in the memory.

Fixed-sized arrays follow the Rust rules. The only exception is the restriction
to constant indexes, that is, you cannot index an array with anything but an
integer literal for now.

Arrays support the index operator, which is explained in detail [here](../../05-operators/00-overview.md).

```rust,no_run,noplaypen
let mut fibbonaci = [0, 1, 1, 2, 3, 5, 8, 13];
let element = fibbonaci[3];
fibbonaci[2] = 1;
```
