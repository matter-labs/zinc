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
single_line_comment = // [~\n .]* \n
multi_line_comment = /* [~*/ .]* */

identifier = ~keyword [A-Za-z_][A-Za-z_0-9]*

type =
  | 'bool'
  | ['uint8' 'uint16' ... 'uint240' 'uint248']
  | ['int8' 'int16' ... 'int240' 'int248']
  | 'field'

keyword =
  // domain
  | 'inputs'
  | 'witness'
  | 'require'
  | 'debug'
  
  // declaration
  | 'let'
  | 'mut'
  
  // control
  | 'for'
  | 'in'
  | 'if'
  | 'else'
  
  // type
  | 'bool'
  | ['uint8' 'uint16' ... 'uint240' 'uint248']
  | ['int8' 'int16' ... 'int240' 'int248']
  | 'field'

  // literal
  | 'true'
  | 'false'

  // operator
  | 'as'

literal =
  | literal_integer
  | literal_boolean
  | literal_string
literal_integer =
  | '0'
  | [1-9][0-9]*
  | 0x[0-9a-fA-F]+
literal_boolean =
  | 'true'
  | 'false'
literal_string =
  | " [~" \" .]* "

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
  | '..'

```
