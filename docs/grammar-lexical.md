# Lexical grammar rules

identifier = (!keyword [A-Za-z_][A-Za-z_0-9]*)

literal = literal_decimal | literal_hexadecimal | literal_boolean

keyword =
  | 'inputs'
  | 'witness'
  | 'require'
  
  | 'let'
  | 'mut'
  | 'type'
  
  | 'for'
  | 'if'
  | 'else'
  | 'match'
  
  | 'field'
  | ('uint1' ... 'uint126')
  | ('int1' ... 'int126')
  | 'bool'

symbol =
  | '{'
  | '}'
  | '['
  | ']'
  | '('
  | ')'
  
  | '='
  
  | '.'
  | ':'
  | ';'
  | ','

  | '+'
  | '-'
  | '*'
  | '/'
  | '%'
  | '\'
  
  | '=='
  | '!='
  | '<='
  | '>='
  | '<'
  | '>'
  
  | '&&'
  | '||'
  | '^^'
  | '!'
