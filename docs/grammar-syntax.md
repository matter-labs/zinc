# Syntax grammar rules

```
root =
  | program
  | library
program = inputs [ witnesses ] statement*
library = statement*

inputs = 'inputs' '{' input* '}'
input = identifier ':' type ';'

witnesses = 'witness' '{' witness* '}'
witness = identifier ':' type ';'

## Statements
statement = 
  | let
  | require
  | debug
let = 'let' [ 'mut' ] identifier [ ':' type ] '=' expression ';'
require = 'require' '(' expression ')' ';'
debug = 'debug' '(' expression ')' ';'

## Expression
expression = term_logical_or ( '||' term_logical_or )*
term_logical_or = term_logical_xor ( '^^' term_logical_xor )*
term_logical_xor = term_logical_and ( '&&' term_logical_and )*
term_logical_and = term_comparison ( ( '!=' '==' '<=' '>=' '<' '>' ) term_comparison )?
term_comparison = term_arithmetic_add_sub ( ('+' | '-') term_arithmetic_add_sub )*
term_arithmetic_add_sub = term_arithmetic_mul_div_rem ( ('*' | '/' | '%') term_arithmetic_mul_div_rem )*
term_arithmetic_mul_div_rem =
  | '(' expression ')'
  | ( '-' | '!' ) term_arithmetic_mul_div_rem
  | literal
  | identifier

```
