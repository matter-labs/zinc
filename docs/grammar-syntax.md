# Syntax grammar rules

```
root = inputs witnesses? statement*

inputs = 'inputs' '{' ( identifier ':' type ',' )* '}'

witnesses = 'witness' '{' ( identifier ':' type ',' )* '}'

## Statements
statement = (
  | empty_statement
  | require_statement
  | let_statement
  | loop_statement
  | type_statement
  | debug_statement
  | expression
) ';'

empty_statement = 
require_statement = 'require' '(' expression ')'
let_statement = 'let' 'mut'? identifier ( ':' type )? '=' expression
loop_statement = 'for' identifier 'in' literal_integer ( '..' | '..=' ) literal_integer block_expression
type_statement = 'type' identifier '=' type
debug_statement = 'debug' '(' expression ')'

## Expression
expression = operand_or ( '||' operand_or )*
operand_or = operand_xor ( '^^' operand_xor )*
operand_xor = operand_and ( '&&' operand_and )*
operand_and = operand_comparison ( ( '==' | '!=' | '>=' | '<=' | '>' | '<' ) operand_comparison )?
operand_comparison = operand_add_sub ( ('+' | '-') operand_add_sub )*
operand_add_sub = operand_mul_div_rem ( ('*' | '/' | '%') operand_mul_div_rem )*
operand_mul_div_rem = operand_as ( 'as' type )*
operand_as =
  | ( '-' | '!' ) operand_as
  | operand_access ( '[' literal_integer ']' | '.' literal_integer | '.' identifier )*
operand_access =
  | literal
  | structure_expression
  | tuple_expression
  | block_expression
  | conditional_expression
  | array_expression

block_expression = '{' statement* expression? '}'

conditional_expression = 'if' expression block_expression ( 'else' ( conditional_expression | block_expression ) )?

array_expression =
  | '[' ( expression ( ',' expression )* )? ']'
  | '[' expression ';' literal_integer ']'

tuple_expression =
  | '(' ')'
  | '(' expression ')'
  | '(' expression ',' ( expression? ( ',' expression )* )? ')'

structure_expression =
  | identifier
  | identifier ( '{' ( identifier ':' type ',' )* '}' )?

```
