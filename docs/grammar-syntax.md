# Syntax grammar rules

```
root = inputs [ witnesses ] statement*

inputs = 'inputs' '{' input* '}'
input = identifier ':' type ';'

witnesses = 'witness' '{' witness* '}'
witness = identifier ':' type ';'

## Statements
statement = (
    | let_statement
    | require_statement
    | debug_statement
    | loop_statement
  ) ';'

let_statement = 'let' [ 'mut' ] identifier [ ':' type ] '=' expression
require_statement = 'require' '(' expression ')'
debug_statement = 'debug' '(' expression ')'
loop_statement = 'for' identifier 'in' literal_integer ( '..' | '..=' ) literal_integer block_expression

## Expression
expression =
  | operator_expression
  | block_expression
  | conditional_expression

operator_expression = operand_or ( '||' operand_or )*
operand_or = operand_xor ( '^^' operand_xor )*
operand_xor = operand_and ( '&&' operand_and )*
operand_and = operand_comparison ( ( '==' | '!=' | '>=' | '<=' | '>' | '<' ) operand_comparison )?
operand_comparison = operand_add_sub ( ('+' | '-') operand_add_sub )*
operand_add_sub = operand_mul_div_rem ( ('*' | '/' | '%') operand_mul_div_rem )*
operand_mul_div_rem = operand_as ( 'as' type )*
operand_as =
  | ( '-' | '!' ) operand_as
  | operand_index ( '[' expression ']' )*
operand_index =
  | literal
  | identifier
  | '(' expression ')'
  | block_expression
  | conditional_expression
  | array_expression
  | tuple_expression

block_expression = '{' statement* expression? '}'

conditional_expression = 'if' expression block_expression [ 'else' ( conditional_expression | block_expression ) ]

array_expression =
  | '[' [ expression [ ',' expression ]* ]? ']'
  | '[' expression ';' literal_integer ']'

tuple_expression =
  | '(' expression ',' expression? [ ',' expression ]* ')'

```

# Operator precedence table

|    Operator     |  Associativity  |
|:---------------:|:---------------:|
| [] .            |  left to right  |
| - !             |      unary      |
| as              |  left to right  |
| * / %           |  left to right  |
| + -             |  left to right  |
| == != <= >= < > |   parenthesis   |
| &&              |  left to right  |
| ^^              |  left to right  |
| ⎮⎮              |  left to right  |
| .. ..=          |     single      |
| =               |     single      |
