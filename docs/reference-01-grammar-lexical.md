# Lexical grammar

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
  | ['u8' 'u16' ... 'u240' 'u248']
  | ['i8' 'i16' ... 'i240' 'i248']
  | 'field'

keyword =
  // built-ins
  | 'inputs'
  | 'witness'
  | 'require'
  | 'debug'

  // declarations
  | 'let'
  | 'mut'
  | 'type'
  | 'struct'

  // controls
  | 'for'
  | 'in'
  | 'while'
  | 'if'
  | 'else'

  // types
  | 'bool'
  | ['u8' 'u16' ... 'u240' 'u248']
  | ['i8' 'i16' ... 'i240' 'i248']
  | 'field'

  // literals
  | 'true'
  | 'false'

  // operators
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
  | '^^'
  | '||'
  | '..'
  | '..='

```
