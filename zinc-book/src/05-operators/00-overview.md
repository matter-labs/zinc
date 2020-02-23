# Operators

Operators of the Zinc language can be divided into several groups:

1. [Arithmetic](./01-arithmetic.md)
2. [Comparison](./02-comparison.md)
3. [Logical](03-logical.md)
4. [Casting](04-casting.md)
5. [Access](05-access.md)
6. [Assignment](06-assignment.md)
7. [Range](07-range.md)

#№ Precedence

The top one is executed first.

|    Operator      |  Associativity  |
|----------------- |-----------------|
|        ::        |  left to right  |
|       [] .       |  left to right  |
|        - !       |      unary      |
|        as        |  left to right  |
|       * / %      |  left to right  |
|        + -       |  left to right  |
|  == != <= >= < > |   parenthesis   |
|        &&        |  left to right  |
|        ^^        |  left to right  |
|        ⎮⎮        |  left to right  |
|      .. ..=      |     single      |
| = += -= *= /= %= |     single      |
