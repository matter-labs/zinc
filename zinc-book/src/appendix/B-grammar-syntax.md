# Syntax grammar (EBNF)

```
file = { module_local_statement } ;

(* Statements *)
module_local_statement =
    const_statement
  | type_statement
  | struct_statement
  | enum_statement
  | fn_statement
  | mod_statement
  | use_statement
  | impl_statement
  | contract_statement
  | empty_statement
;

function_local_statement =
    let_statement
  | const_statement
  | loop_statement
  | empty_statement
  | expression, [ ';' ]
;

implementation_local_statement =
    const_statement
  | fn_statement
  | empty_statement
;

contract_local_statement =
    field_statement
  | const_statement
  | fn_statement
  | empty_statement
;

field_statement = [ 'pub' ], [ 'extern' ], identifier, ':', type, ';' ;

type_statement = [ 'pub' ], 'type', identifier, '=', type, ';' ;

struct_statement = [ 'pub' ], 'struct', '{', field_list, '}' ;

enum_statement = [ 'pub' ], 'enum', '{', variant_list, '}' ;

fn_statement = [ 'pub' ], [ 'const' ], 'fn', identifier, '(', binding_list, ')', [ '->', type ], block_expression ;

mod_statement = [ 'pub' ], 'mod', identifier, ';' ;

use_statement = [ 'pub' ], 'use', path_expression, [ 'as', identifier ], ';' ;

impl_statement = 'impl', identifier, '{', { implementation_local_statement }, '}' ;

const_statement = [ 'pub' ], 'const', identifier, ':', type, '=', expression, ';' ;

let_statement = 'let', binding, '=', expression, ';' ;

loop_statement = 'for', identifier, 'in', expression, [ 'while', expression ], block_expression ;

contract_statement = 'contract', '{', { contract_local_statement }, '}' ;

empty_statement = ';' ;

(* Expressions *)
expression = operand_assignment, [ '=' | '+=' | '-=' | '*=' | '/=' | '%=' | '<<=' | '>>=' | '|=' | '^=' | '&=', operand_assignment ] ;
operand_assignment = operand_range, [ '..' | '..=', operand_range ] ;
operand_range = operand_or, { '||', operand_or } ;
operand_or = operand_xor, { '^^', operand_xor } ;
operand_xor = operand_and, { '&&', operand_and } ;
operand_and = operand_comparison, [ '==' | '!=' | '>=' | '<=' | '>' | '<', operand_comparison ] ;
operand_comparison = operand_bitwise_or, { '|', operand_bitwise_or } ;
operand_bitwise_or = operand_bitwise_xor, { '^', operand_bitwise_xor } ;
operand_bitwise_xor = operand_bitwise_and, { '&', operand_bitwise_and } ;
operand_bitwise_and = operand_bitwise_shift, { '<<' | '>>', operand_bitwise_shift } ;
operand_bitwise_shift = operand_add_sub, { '+' | '-', operand_add_sub } ;
operand_add_sub = operand_mul_div_rem, { '*' | '/' | '%', operand_mul_div_rem } ;
operand_mul_div_rem = operand_as, { 'as', type } ;
operand_as = { '-' | '~' | '!' }, operand_access ;
operand_access = operand_path, {
    '[', expression, ']'
  | '.', integer | identifier
  | [ '!' ], '(', expression_list, ')'
} ;
operand_path = operand_terminal, { '::', operand_terminal }, [ structure_expression ] ;
operand_terminal =
    tuple_expression
  | block_expression
  | array_expression
  | conditional_expression
  | match_expression
  | literal
  | identifier
  | alias
;

expression_list = [ expression, { ',', expression } | ',' ] ;

block_expression = '{', { function_local_statement }, [ expression ], '}' ;

conditional_expression = 'if', expression, block_expression, [ 'else', conditional_expression | block_expression ] ;

match_expression = 'match', expression, '{', { pattern_match, '=>', expression, ',' }, '}' ;

array_expression =
    '[', [ expression, { ',', expression } ] ']'
  | '[', expression, ';', integer, ']'
;

tuple_expression =
    '(', ')'
  | '(', expression, ')'
  | '(', expression, ',', [ expression, { ',', expression } ], ')'
;

structure_expression = '{', field_list, '}';

(* Attributes *)
attribute = '#', [ '!' ], '[', attribute_element_list, ']' ;
attribute_element = 
    identifier
  | identifier, '=', literal
  | identifier, '(', attribute_element_list, ')' ;
attribute_element_list = [ attribute_element, { ',', attribute_element } | ',' ] ;

(* Parts *)
alias = 'crate' | 'super' | 'self' | 'Self'

type =
    '(', ')'
  | 'bool'
  | 'u8' | 'u16' | 'u24' | 'u32' | 'u40' | 'u48' | 'u56' | 'u64'
  | 'u72' | 'u80' | 'u88' | 'u96' | 'u104' | 'u112' | 'u120' | 'u128'
  | 'u136' | 'u144' | 'u152' | 'u160' | 'u168' | 'u176' | 'u184' | 'u192'
  | 'u200' | 'u208' | 'u216' | 'u224' | 'u232' | 'u240' | 'u248' | 'field'
  | 'i8' | 'i16' | 'i24' | 'i32' | 'i40' | 'i48' | 'i56' | 'i64'
  | 'i72' | 'i80' | 'i88' | 'i96' | 'i104' | 'i112' | 'i120' | 'i128'
  | 'i136' | 'i144' | 'i152' | 'i160' | 'i168' | 'i176' | 'i184' | 'i192'
  | 'i200' | 'i208' | 'i216' | 'i224' | 'i232' | 'i240' | 'i248'
  | 'field'
  | '[', type, ';', expression, ']'
  | '(', type, { ',', type }, ')'
  | identifier | alias, { '::', identifier | alias }
;

pattern_match =
    boolean
  | integer
  | identifier
  | operand_path
  | '_'
;

binding = pattern_binding, [ ':', type ] ;
binding_list = [ binding, { ',', binding } | ',' ] ;
pattern_binding =
    [ 'mut' ], identifier
  | ( pattern_binding, { ',', pattern_binding } | ',' ) 
  | '(', ')'
  | '_'
;

field = identifier, ':', type ;
field_list = [ field, { ',', field } | ',' ] ;

variant = identifier, '=', integer ;
variant_list = [ variant, { ',', variant } | ',' ] ;
```
