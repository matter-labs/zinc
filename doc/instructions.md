# Bytecode Reference

## Contents

- [Bytecode Overview](#Bytecode-Overview)
    - [Parameter Encoding](#Parameter-Encoding)
- [Instructions](#Instructions)
    - [General](#General)
    - [Stack](#Stack)
    - [Arithmetic](#Arithmetic)
    - [Boolean](#Boolean)
    - [Comparison](#Comparison)
    - [Conditional](#Conditional)
    - [Loops](#Loops)

## Bytecode Overview

Bytecode is a sequence of operators encoded into byte array.

Operators are encoded as single-byte operation code optionally followed by operator parameters.

Operator can *consume* (remove from top of the stack) some elements and *push* another ones.

### Parameter Encoding

Parameters that are placed into bytecode are encoded using sort of
[variable-length quantity](https://en.wikipedia.org/wiki/Variable-length_quantity) encoding.

## Instructions

### General

| Operation | Parameters | Stack Input | Stack Output  | Description |
|-----------|------------|-------------|---------------|-------------|
| `noop`    |     -      |      -      |       -       | Does nothing.

### Stack

Stack elements are zero-indexed, i.e. `0` is the top element, `1` is then next one.

| Operation | Parameters | Stack Input | Stack Output  | Description |
|-----------|------------|-------------|---------------|-------------|
| `push`    |  `value`   |      -      |       1       | Pushes encoded value.
| `pop`     |  `count`   |      1      |       -       | Consumes `count` elements.
| `copy`    |  `index`   |      -      |       1       | Copies and pushes element at `index` from top of the stack.

### Arithmetic

| Operation | Parameters | Stack Input | Stack Output  | Description |
|-----------|------------|-------------|---------------|-------------|
| `add`     |     -      |      2      |       1       | Calculates sum
| `sub`     |     -      |      2      |       1       | Calculates difference
| `mul`     |     -      |      2      |       1       | Calculates production
| `div`     |     -      |      2      |       1       | Calculates quotient (floored)
| `rem`     |     -      |      2      |       1       | Calculates remainder
| `neg`     |     -      |      1      |       1       | Calculates negated value

### Boolean

| Operation | Parameters | Stack Input | Stack Output  | Description |
|-----------|------------|-------------|---------------|-------------|
| `not`     |     -      |      1      |       1       | Calculates logical negation
| `and`     |     -      |      2      |       1       | Calculates conjunction
| `or`      |     -      |      2      |       1       | Calculates disjunction
| `xor`     |     -      |      2      |       1       | Calculates exclusive disjunction

### Comparison

| Operation | Parameters | Stack Input | Stack Output  | Description |
|-----------|------------|-------------|---------------|-------------|
| `lt`      |     -      |      2      |       1       | Calculates 'less than' (<)
| `le`      |     -      |      2      |       1       | Calculates 'less or equal' (<=)
| `eq`      |     -      |      2      |       1       | Calculates 'equals' (==)
| `ne`      |     -      |      2      |       1       | Calculates 'not equals' (!=)
| `ge`      |     -      |      2      |       1       | Calculates 'greater or equal' (>=)
| `gt`      |     -      |      2      |       1       | Calculates 'greater than' (>)

### Conditional

| Operation | Parameters | Stack Input           | Stack Output  | Description |
|-----------|------------|-----------------------|---------------|-------------|
| `cs`      |     -      | `condition`, `t`, `f` |       1       | Returns second argument if condition is true, third argument otherwise.

### Loops

| Operation    | Parameters   | Stack Input | Stack Output  | Description |
|--------------|--------------|-------------|---------------|-------------|
| `loop_begin` | `iterations` |      -      |       -       | Marks a beginning of the loop block, setting number of iterations. 
| `loop_begin` |      -       |      -      |       -       | Marks an ending of the loop block.
