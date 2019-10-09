# Array

Fixed-sized arrays follow the Rust rules.

The only exception is the temporary restriction to constant indexes, that is,
you cannot index an array with a variable for now.

## Indexing

Arrays support an index operator:

```jab
let element = fibbonaci[3];
fibbonaci[2] = 1;
```

## Examples

```jab
let fibbonaci: [u8; 5] = [1, 1, 2, 3, 5];
let mut a: [u8, 3] = [1, 2, 3]; // initialized with all zeros
```
