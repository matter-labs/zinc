# Syntax grammar rules

root =
  | program
  | library

program =
    inputs
    [ witness ]
    statement*

library =
    statement*

inputs =
    'inputs' '{'
    ( identifier ':' type ';' )*
    '}'

witness =
    'witness' '{'
    ( identifier ':' type ';' )*
    '}'

## Statements
statement = 
  | let
  | require

let = 'let' [ 'mut' ] identifier ':' type '=' expression ';'

require = 'require' '(' boolean_expression ')' ';'

## Expressions

expression = 
  | boolean_expression
  | arithmetic_expression

boolean_expression = 
  | boolean_or_term ( '||' boolean_or_term )*

boolean_or_term =
  | boolean_xor_factor ( '^^' boolean_xor_factor )*

boolean_xor_factor =
  | boolean_and_factor ( '&&' boolean_and_factor )*

boolean_and_factor =
  | arithmetic_expression ( '!=' '==' '<=' '>=' '<' '>' ) arithmetic_expression
  | '(' boolean_expression ')'
  | '!' boolean_and_factor
  | literal_boolean
  | identifier

arithmetic_expression = 
  | arithmetic_term ( ('+' | '-') arithmetic_term )*

arithmetic_term =
  | arithmetic_factor ( ('*' | '/' | '%') arithmetic_factor )*

arithmetic_factor =
  | '(' arithmetic_expression ')'
  | '-' arithmetic_factor
  | literal_integer
  | identifier
