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
require = 'require' '(' boolean_expression ')' ';'
debug = 'debug' '(' expression ')' ';'

## Expressions
expression = 
  | boolean_expression
  | arithmetic_expression

boolean_expression = boolean_or_term ( '||' boolean_or_term )*
boolean_or_term = boolean_xor_term ( '^^' boolean_xor_term )*
boolean_xor_term = boolean_and_factor ( '&&' boolean_and_factor )*
boolean_and_factor =
  | '(' boolean_expression ')'
  | '!' boolean_and_factor
  | arithmetic_expression ( '!=' '==' '<=' '>=' '<' '>' ) arithmetic_expression
  | literal_boolean
  | identifier

arithmetic_expression = arithmetic_term ( ('+' | '-') arithmetic_term )*
arithmetic_term = arithmetic_factor ( ('*' | '/' | '%') arithmetic_factor )*
arithmetic_factor =
  | '(' arithmetic_expression ')'
  | '-' arithmetic_factor
  | literal_integer
  | identifier

```
