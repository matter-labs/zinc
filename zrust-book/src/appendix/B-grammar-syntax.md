# Syntax grammar

These are the ZRust syntax grammar rules in the EWBF notation.

```
(* Domain *)
file = binary | library ;

binary = { outer_statement } ;

library = { outer_statement } ;

type =
    'bool'
  | 'u8' | 'u16' | 'u24' | 'u32' | 'u40' | 'u48' | 'u56' | 'u64'
  | 'u72' | 'u80' | 'u88' | 'u96' | 'u104' | 'u112' | 'u120' | 'u128'
  | 'u136' | 'u144' | 'u152' | 'u160' | 'u168' | 'u176' | 'u184' | 'u192'
  | 'u200' | 'u208' | 'u216' | 'u224' | 'u232' | 'u240' | 'u248' | 'field'
  | 'i8' | 'i16' | 'i24' | 'i32' | 'i40' | 'i48' | 'i56' | 'i64'
  | 'i72' | 'i80' | 'i88' | 'i96' | 'i104' | 'i112' | 'i120' | 'i128'
  | 'i136' | 'i144' | 'i152' | 'i160' | 'i168' | 'i176' | 'i184' | 'i192'
  | 'i200' | 'i208' | 'i216' | 'i224' | 'i232' | 'i240' | 'i248'
  | '[', type, ';', integer, ']'
  | '(', { type }, ')'
  | identifier
;

field = identifier, ':', type ;
field_list = [ field, { ',', field } ] ;

variant = identifier, '=', integer ;
variant_list = [ variant, { ',', variant } ] ;

(* Statements *)
outer_statement =
    type_statement
  | struct_statement
  | enum_statement
  | fn_statement
  | mod_statement
  | use_statement

inner_statement =
    empty_statement
  | let_statement
  | loop_statement
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

block_expression = '{', { inner_statement }, [ expression ], '}' ;

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

path_expression = identifier, [ '!' ], { '::', identifier } ;

```
