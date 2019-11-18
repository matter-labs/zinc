# B - Syntax grammar

```text
(* Domain *)
file = binary | library ;

binary = inputs, witnesses, outputs, { statement } ;

library = { statement } ;

inputs = 'input', '{', field_list, '}' ;

witnesses = 'witness', '{', field_list, '}' ;

outputs = 'output', '{', field_list, '}' ;

type =
    'bool'
  | 'u8' | 'u16' ... 'u240' | 'u248'
  | 'i8' | 'i16' ... 'i240' | 'i248'
  | 'field'
  | '[', type, ';', integer, ']'
  | '(', { type }, ')'
  | identifier
;

field = identifier, ':', type ;
field_list = [ field, { ',', field } ] ;

variant = identifier, '=', integer ;
variant_list = [ variant, { ',', variant } ] ;

(* Statements *)
statement =
    empty_statement
  | let_statement
  | loop_statement
  | type_statement
  | struct_statement
  | enum_statement
  | fn_statement
  | mod_statement
  | use_statement
  | expression
';' ;

empty_statement = ;
let_statement = 'let', [ 'mut' ], identifier, [ ':', type ], '=', expression ;
loop_statement = 'for', identifier, 'in', integer, '..' | '..=', integer, [ 'while', expression ], block_expression ;
type_statement = 'type', identifier, '=', type ;
struct_statement = 'struct', field_list ;
enum_statement = 'enum', variant_list ;
fn_statement = 'fn', identifier, '(', field_list, ')', [ '->', type ], block_expression ;
mod_statement = 'mod', identifier ;
use_statement = 'use', path_expression ;

(* Expressions *)
expression = operand_or, { '||', operand_or } ;
operand_or = operand_xor, { '^^', operand_xor } ;
operand_xor = operand_and, { '&&', operand_and } ;
operand_and = operand_comparison, [ '==' | '!=' | '>=' | '<=' | '>' | '<', operand_comparison ] ;
operand_comparison = operand_add_sub, { '+' | '-', operand_add_sub } ;
operand_add_sub = operand_mul_div_rem, { '*' | '/' | '%', operand_mul_div_rem } ;
operand_mul_div_rem = operand_as, { 'as', type } ;
operand_as =
    '-' | '!', operand_as
  | operand_access, { '[', integer, ']' | '.', integer | '.', identifier | '(', expression_list, ')' }
operand_access
    tuple_expression
  | block_expression
  | array_expression
  | conditional_expression
  | match_expression
  | struct_expression
  | literal
  | path_expression
;

expression_list = [ expression, { ',', expression } ] ;

block_expression = '{', { statement }, [ expression ], '}' ;

conditional_expression = 'if', expression, block_expression, [ 'else', conditional_expression | block_expression ] ;

match_expression = 'match', expression, '{', { literal, '=>', expression, ',' }, '}' ;

array_expression =
    '[', [ expression, { ',', expression } ] ']'
  | '[', expression, ';', integer, ']'
;

tuple_expression =
    '(', ')'
  | '(', expression, ')'
  | '(', expression, ',', [ expression, { ',', expression } ], ')'
;

struct_expression = 'struct', path_expression, [ '{', field_list, '}' ] ;

path_expression = identifier, { '::', identifier } ;
```

