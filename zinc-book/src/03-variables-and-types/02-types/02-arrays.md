# Arrays

Arrays are collections of values of the same type sequentially stored in the memory.

Arrays support the index and slice operators, which is explained in detail
[here](../../04-operators/06-access.md).

```rust,no_run,noplaypen
let mut fibbonaci = [0, 1, 1, 2, 3, 5, 8, 13];
let element = fibbonaci[3];
fibbonaci[2] = 1;
```

> There is a minor restriction for arrays at the current language state. Arrays
> cannot be indexed with a witness value, but only with a constant or
> witness-independent variable.
