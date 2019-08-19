# Syntax grammar rules

```
root = inputs [ witnesses ] statement*

inputs = 'inputs' '{' input* '}'
input = identifier ':' type ';'

witnesses = 'witness' '{' witness* '}'
witness = identifier ':' type ';'

## Statements
statement = 
  | 'let' [ 'mut' ] identifier [ ':' type ] '=' expression ';'
  | 'require' '(' expression ')' ';'
  | 'debug' '(' expression ')' ';'

## Expression
expression = operand_or ( '||' operand_or )*
operand_or = operand_xor ( '^^' operand_xor )*
operand_xor = operand_and ( '&&' operand_and )*
operand_and = operand_comparison ( ( '==' | '!=' | '>=' | '<=' | '>' | '<' ) operand_comparison )?
operand_comparison = operand_add_sub ( ('+' | '-') operand_add_sub )*
operand_add_sub = operand_mul_div_rem ( ('*' | '/' | '%') operand_mul_div_rem )*
operand_mul_div_rem = operand_as ( 'as' type )+
operand_as =
  | ( '-' | '!' ) operand_as
  | '(' expression ')'
  | literal
  | identifier

```
