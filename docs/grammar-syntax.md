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

require = 'require' '(' boolean_expression ')' ';'

## Expressions
arithmetic_expression = 
  | arithmetic_term ( ('+' | '-') arithmetic_term )*

arithmetic_term =
  | arithmetic_factor ( ('*' | '/' | '%') arithmetic_factor )*

arithmetic_factor =
  | identifier
  | constant
  | '-' arithmetic_factor
  | '(' arithmetic_expression ')'

boolean_expression = 
  | boolean_term ( '||' boolean_term )*

boolean_term =
  | boolean_factor ( '&&' boolean_factor )*

boolean_factor =
  | arithmetic_expression ( '!=' '==' '<=' '>=' '<' '>' ) arithmetic_expression
  | identifier
  | constant
  | '!' boolean_factor
  | '(' boolean_expression ')'
