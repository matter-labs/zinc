program = inputs [ witness ] statement*

inputs = 'inputs' '{' ( name ':' type ';' )* '}'

witness = 'witness' '{' ( name ':' type ';' )* '}'

# Statements
statement = (require | let | if | for)

require = 'require' '(' bool_expr ')' ';'

let = 'let' [ 'mut' ] variable ':' type '=' expr ';'

if = 'if' bool_expr '{' statement* '}' [ 'else' '{' statement* '}' ]

for = 'for' variable 'in' const '..' const '{' statement* '}' 

# Expressions
expr = arith_expr | bool_expr

arith_expr =
    '(' arith_expr ')'
  | unary_arith_op arith_expr
  | arith_expr binary_arith_op arith_expr
  | variable
  | const
    
bool_expr =
    '(' bool_expr ')'
  | unary_bool_op bool_expr
  | arith_expr comp_op arith_expr
  | bool_expr binary_bool_op bool_expr
  | variable
  | const

# Operators
comp_op = '>=' | '<=' | '>' | '<' | '==' | '!='

unary_arith_op = '-'

binary_arith_op = '+' | '-' | '*' | '/'

unary_bool_op = '!'

binary_bool_op = '&&' | '||' | '^^'

# Lexemes
type =
    'field'
  | 'uint8'
  | 'int8'
  | 'uint16'
  | 'int16'
  | 'uint32'
  | 'int32'
  | 'uint64'
  | 'int64'
  | 'uint128'
  | 'int128'
  | 'bool'
  | 'struct'
  | 'memory_vector' '<' type ',' const '>'
  | 'storage_vector' '<' type ',' const '>'

variable = (!keyword [A-Za-z_][A-Za-z_0-9]*)

const = [0-9]+

keyword =
    'inputs'
  | 'witness'
  | 'require'
  | 'let'
  | 'mut'
  | 'field'
  | 'uint8'
  | 'int8'
  | 'uint16'
  | 'int16'
  | 'uint32'
  | 'int32'
  | 'uint64'
  | 'int64'
  | 'uint128'
  | 'int128'
  | 'bool'
  | 'struct'
  | 'memory_vector'
  | 'storage_vector'
