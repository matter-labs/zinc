# Operators

Operators of the Zinc language can be divided into several groups:

- [Arithmetic](./01-arithmetic.md)
- [Bitwise](./02-bitwise.md)
- [Comparison](./03-comparison.md)
- [Logical](./04-logical.md)
- [Casting](./05-casting.md)
- [Access](./06-access.md)
- [Range](./07-range.md)
- [Assignment](./08-assignment.md)

#№ Precedence

The top one is executed first.

|              Operator              |    Associativity    |
|------------------------------------|---------------------|
| ::                                 | left to right       |
| [] .                               | left to right       |
| - ~ !                              | unary               |
| as                                 | left to right       |
| * / %                              | left to right       |
| + -                                | left to right       |
| << >>                              | left to right       |
| &                                  | left to right       |
| ^                                  | left to right       |
| ⎮                                 | left to right       |
| == != <= >= < >                    | require parentheses |
| &&                                 | left to right       |
| ^^                                 | left to right       |
| ⎮⎮                               | left to right       |
| .. ..=                             | require parentheses |
| = += -= *= /= %= ⎮= ^= &= <<= >>= | require parentheses |
