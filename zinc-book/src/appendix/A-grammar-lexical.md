# Lexical grammar (EBNF)

```
lexeme = comment | identifier | keyword | literal | symbol | EOF ;

comment = single_line_comment | multi_line_comment ;
single_line_comment = '//', ( ? ANY ? - '\n' | EOF ), '\n' | EOF ;
multi_line_comment = '/*', ( ? ANY ? - '*/' ), '*/' ;

identifier = (
    alpha, { alpha | digit | '_' }
  | '_', alpha, { alpha }
- keyword ) ;

keyword =
    'let'
  | 'mut'
  | 'const'
  | 'type'
  | 'struct'
  | 'enum'
  | 'fn'
  | 'mod'
  | 'use'
  | 'impl'
  | 'contract'
  | 'pub'

  | 'for'
  | 'in'
  | 'while'
  | 'if'
  | 'else'
  | 'match'

  | 'bool'
  | 'u8' | 'u16' | 'u24' | 'u32' | 'u40' | 'u48' | 'u56' | 'u64'
  | 'u72' | 'u80' | 'u88' | 'u96' | 'u104' | 'u112' | 'u120' | 'u128'
  | 'u136' | 'u144' | 'u152' | 'u160' | 'u168' | 'u176' | 'u184' | 'u192'
  | 'u200' | 'u208' | 'u216' | 'u224' | 'u232' | 'u240' | 'u248'
  | 'i8' | 'i16' | 'i24' | 'i32' | 'i40' | 'i48' | 'i56' | 'i64'
  | 'i72' | 'i80' | 'i88' | 'i96' | 'i104' | 'i112' | 'i120' | 'i128'
  | 'i136' | 'i144' | 'i152' | 'i160' | 'i168' | 'i176' | 'i184' | 'i192'
  | 'i200' | 'i208' | 'i216' | 'i224' | 'i232' | 'i240' | 'i248'
  | 'field'

  | 'true'
  | 'false'

  | 'as'

  | 'crate'
  | 'super'
  | 'self'
  | 'Self'

  | 'static'
  | 'ref'
  | 'extern'
  | 'return'
  | 'loop'
  | 'break'
  | 'continue'
  | 'trait'
;

literal = boolean | integer | string ;
boolean = 'true' | 'false' ;
integer =
    '0'
  | '0b', binary_digit | '_', { binary_digit | '_' }
  | '0o', octal_digit | '_', { octal_digit | '_' }
  | decimal_digit - '0', { decimal_digit | '_' }
  | '0x', hexadecimal_digit | '_', { hexadecimal_digit | '_' }
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
  | '|'
  | '&'
  | '^'
  | '~'
  | '#'
  | '<<'
  | '>>'
  | '+='
  | '-='
  | '*='
  | '/='
  | '%='
  | '|='
  | '&='
  | '^='
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
  | '<<='
  | '>>='
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

binary_digit = '0' | '1' ;

octal_digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' ;

decimal_digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;

hexadecimal_digit = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
  | 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
  | 'a' | 'b' | 'c' | 'd' | 'e' | 'f'
;

```

### The Zinc alphabet

|    Group     |                     Characters                      |
|--------------|-----------------------------------------------------|
| whitespaces  | \t \n \r <Space>                                    |
| lowercase    | A B C D E F G H I J K L M N O P Q R S T U V W X Y Z |
| uppercase    | a b c d e f g h i j k l m n o p q r s t u v w x y z |
| numbers      | 0 1 2 3 4 5 6 7 8 9                                 |
| symbols      | + - * / % < = > ⎮ & ^ _ ! ~ ( ) [ ] { } " , . : ; # |
