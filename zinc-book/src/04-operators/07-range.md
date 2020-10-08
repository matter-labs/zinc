# Range operators

### Range

`..` is a binary operator.

Range operator is used only for loop bounds or array slicing.

The operator can accept operands of different integer types. The result will
be signed if any of the operands if signed, and the bitlength will be enough
to contain the greater range bound.

*Accepts*
1. Constant integer expression
2. Constant integer expression

*Returns* a temporary range element to be used as a slice or loop range.

### Inclusive range

`..=` is a binary operator.

The same as the above, but the right range bound is inclusive.

*Accepts*
1. Constant integer expression
2. Constant integer expression

*Returns* a temporary range element to be used as a slice or loop range.
