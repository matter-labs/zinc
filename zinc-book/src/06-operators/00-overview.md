# Operators

Operators do not perform any kind of overflow checking. If an overflow happens,
the Zinc VM will fail during proof generation.

## Assignment

`=`

**Accepts**
1. Place expression (the descriptor of a memory place)
2. Value expression

**Returns** `()`.

## Arithmetic

#### Addition

`+`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

#### Subtraction

`-`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

#### Negation

`-`

**Accepts**
1. Integer expression

**Returns** the integer result.

#### Multiplication

`*`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

#### Division

`/`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

#### Remainder

`%`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the integer result.

## Logical

#### Logical OR

`||`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

#### Logical XOR

`^^`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

#### Logical AND

`&&`

**Accepts**
1. Boolean expression
2. Boolean expression

**Returns** the boolean result.

#### Logical NOT

`!`

**Accepts**
1. Boolean expression

**Returns** the boolean result.

## Comparison

#### Equality

`==`

**Accepts**
1. Integer **or** boolean expression
2. Integer **or** boolean expression

**Returns** the boolean result.

#### Non-equality

`!=`

**Accepts**
1. Integer **or** boolean expression
2. Integer **or** boolean expression

**Returns** the boolean result.

#### Lesser or equals

`<=`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

#### Greater or equals

`>=`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

#### Lesser

`<`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

#### Greater

`>`

**Accepts**
1. Integer expression
2. Integer expression

**Returns** the boolean result.

## Casting

`as`

**Accepts**
1. Integer expression
2. Integer type

**Returns** the integer result.

Casting allowed:

- integers to types of greater bitlength
- enums to integers of enough bitlength

```rust,no_run,noplaypen
let a = -1; // inference
let b: u16 = a as u16; // ok, casted to the opposite sign with greater bitlength 
let c: u8 = Order::First; // casting to an integer of enough bitlength
```

## Access

#### Path resolution

`::`

**Accepts**
1. Namespace identifier (module, structure, enumeration)
2. Identifier

**Returns** the second operand.

#### Array indexing

`[]`

**Accepts**
1. Array expression
2. Integer expression

**Returns** an array element or error.

#### Field access

`.`

**Accepts**
1. Tuple or structure expression
2. Tuple index or structure field name

**Returns** a tuple or structure element or error.

## Range

#### Exclusive range (TODO)

`..`

#### Inclusive range (TODO)

`..=`
