# Lexical grammar rules

```
lexeme =
  | comment
  | identifier
  | keyword
  | literal
  | symbol

comment =
  | single_line_comment
  | multi_line_comment
single_line_comment = '//' (!\n .)* \n
multi_line_comment = '/*' (!'*/' .)* '*/'

identifier = !keyword ([A-Za-z_][A-Za-z_0-9])*

keyword =
  // domain
  | 'inputs'
  | 'witness'
  | 'require'
  | 'debug'
  
  // declaration
  | 'let'
  | 'mut'
  | 'type'
  
  // control
  | 'for'
  | 'if'
  | 'else'
  | 'match'
  
  // types
  | 'field'
  | ('uint1' ... 'uint126')
  | ('int1' ... 'int126')
  | 'bool'

literal =
  | literal_integer
  | literal_boolean
literal_integer =
  | [0-9][1-9]*
  | '0x' [0-9a-fA-F]+
literal_boolean =
  | 'true'
  | 'false'

symbol =
  // simple
  | '('
  | ')'
  | '['
  | ']'
  | '{'
  | '}'    
  | '.'
  | ':'
  | ';'
  | ','
  | '='
  | '+'
  | '-'
  | '*'
  | '/'
  | '%'
  | '\'
  | '!'
  | '<'
  | '>'
  
  // complex
  | '=='
  | '!='
  | '<='
  | '>='  
  | '&&'
  | '||'
  | '^^'

```
