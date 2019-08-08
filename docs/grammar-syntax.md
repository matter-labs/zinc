# Syntax grammar rules

program =
    inputs
    [ witness ]
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
    let
  | require

let = 'let' [ 'mut' ] identifier ':' type '=' expression ';'

require = 'require' '(' expression ')' ';'

## Expressions
expression = 
    '(' expression ')'
  | ( unary_operator expression )
  | ( expression binary_operator expression )
  | identifier
  | constant
