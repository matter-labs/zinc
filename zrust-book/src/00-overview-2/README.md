# Operators

## Path resolution

`::`

**Accepts** 1. Module or enum 2. Identifier or module

**Returns** the second operand.

## Array indexing

`[]`

**Accepts** 1. Array expression 2. Integer literal

**Returns** the array element or an out of range error.

**Note**: only integer literals can be array indexes now.

## Field access

`.`

**Accepts** 1. Tuple expression 2. Integer literal

**Returns** the tuple or structure element or an nonexistent field error.

## Unary minus

`-`

**Accepts** 1. Integer expression

**Returns** the integer result.

## Logical NOT

`!`

**Accepts** 1. Boolean expression

**Returns** the boolean result.

## Casting

`as`

**Accepts** 1. Integer expression 2. Integer type

**Returns** the integer result.

Casting allowed:

* integers to types of greater bitlength
* enums to integers of enough bitlength

```rust
let a = -1; // `i8`, after a cast using the unary minus and the `let` inference
let b: u16 = a as u16; // ok, casted to the opposite sign with greater bitlength 
let c: u8 = Order::FIRST; // ok, enum implicit casting to the enough bitlength
```

## Multiplication

`*`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the integer result.

## Division

`/`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the integer result.

## Remainder

`%`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the integer result.

## Addition

`+`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the integer result.

## Subtraction

`-`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the integer result.

## Equality

`==`

**Accepts** 1. Integer **or** boolean expression 2. Integer **or** boolean expression

**Returns** the boolean result.

## Non-equality

`!=`

**Accepts** 1. Integer **or** boolean expression 2. Integer **or** boolean expression

**Returns** the boolean result.

## Lesser or equals

`<=`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the boolean result.

## Greater or equals

`>=`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the boolean result.

## Lesser

`<`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the boolean result.

## Greater

`>`

**Accepts** 1. Integer expression 2. Integer expression

**Returns** the boolean result.

## Logical AND

`&&`

**Accepts** 1. Boolean expression 2. Boolean expression

**Returns** the boolean result.

## Logical XOR

`^^`

**Accepts** 1. Boolean expression 2. Boolean expression

**Returns** the boolean result.

## Logical OR

`||`

**Accepts** 1. Boolean expression 2. Boolean expression

**Returns** the boolean result.

## Exclusive range \(TODO\)

`..`

## Inclusive range \(TODO\)

`..=`

## Assignment

`=`

**Accepts** 1. Place expression 2. Value expression

**Returns** `()`.

