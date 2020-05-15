# Access operators

#### Path resolution

`::` is a binary operator.

**Accepts**
1. Namespace identifier (module, structure, enumeration)
2. Item identifier (module, type, variable, constant etc.)

**Returns** the second operand.

#### Array indexing

`[]` is a binary operator.

**Accepts**
1. Array expression
2. Integer or range expression

**Returns** an array element (if the 2nd operand is an integer) or a sub-array
(if the 2nd operand is a range).

#### Field access

`.` is a binary operator.

**Accepts**
1. Tuple or structure expression
2. Tuple index or structure field name

**Returns** a tuple or structure element.
