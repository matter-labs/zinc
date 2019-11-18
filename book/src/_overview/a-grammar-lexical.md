# A - Lexical grammar

```text
lexeme = comment | identifier | keyword | literal | symbol | EOF ;

comment = single_line_comment | multi_line_comment ;
single_line_comment = '//', ( ? ANY ? - '\n' ), '\n' ;
multi_line_comment = '/*', ( ? ANY ? - '*/' ), '*/' ;

identifier = (
    alpha, { alpha | digit | '_' }
  | '_', alpha, { alpha }
- keyword ) ;

keyword =
    'input'
  | 'witness'
  | 'output'

  | 'let'
  | 'mut'
  | 'type'
  | 'struct'
  | 'enum'
  | 'fn'
  | 'mod'
  | 'use'

  | 'for'
  | 'in'
  | 'while'
  | 'if'
  | 'else'
  | 'match'

  | 'bool'
  | 'u8' | 'u16' ... 'u240' | 'u248'
  | 'i8' | 'i16' ... 'i240' | 'i248'
  | 'field'

  | 'true'
  | 'false'

  | 'as'
;

literal = boolean | integer | string ;
boolean = 'true' | 'false' ;
integer =
    '0'
  | digit - '0', { digit }
  | '0x', hex_digit, { hex_digit}
;
string = '"', { ANY - '"' | '\', ANY }, '"' ;

symbol =
    '('
  | ')'
  | '['
  | ']'
  | '{'
  | '}'
  | '_'
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
  | '::'
  | '=='
  | '!='
  | '<='
  | '>='  
  | '&&'
  | '^^'
  | '||'
  | '..'
  | '..='
  | '=>'
  | '->'
;

alpha =
    'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G'
  | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N'
  | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U'
  | 'V' | 'W' | 'X' | 'Y' | 'Z' 
  | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g'
  | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
  | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u'
  | 'v' | 'w' | 'x' | 'y' | 'z'
;

digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;

hex_digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
  | 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
  | 'a' | 'b' | 'c' | 'd' | 'e' | 'f'
;
```

