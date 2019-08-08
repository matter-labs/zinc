# Lexical grammar rules

keyword =
    'inputs'
  | 'witness'
  | 'require'
  | 'let'
  | 'mut'
  | 'field'
  | ('uint1' ... 'uint253')
  | ('int1' ... 'int253')
  | 'bool'

type =
    'field'
  | ('uint1' ... 'uint253')
  | ('int1' ... 'int253')
  | 'bool'

identifier = (!keyword [A-Za-z_][A-Za-z_0-9]*)

constant =
    [1-9][0-9_]*
  | ('0x' [0-9a-fA-F_]+
  | '0'

binary_operator =
    '+'
  | '-'
  | '*'
  | '/'
  | '>='
  | '<='
  | '>'
  | '<'
  | '=='
  | '!='
  | '&&'
  | '||'
  | '^^'

unary_operator =
    '-'
  | '!'
