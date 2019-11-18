# Precedence

The top one is executed the first.

```text
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
|         =        |     single      |
```

