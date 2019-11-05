# Bytecode Specification

## Contents

- [Overview](#Overview)
- [Operators](#Operators)
    - [General](#General)
    - [Stack](#Stack)
    - [Arithmetic](#Arithmetic)

## Overview

Bytecode is a sequence of operators encoded into byte array.

Operators are encoded as single-byte operation code optionally followed by operator arguments.
Number and encoding of arguments are specified for each operator individually.

Operator can *consume* (remove from top of the stack) some elements and *push* another ones.

### Argument encoding

Most arguments are encoded using [VLQ](https://en.wikipedia.org/wiki/Variable-length_quantity).

## Operators

### General

| Operation | Arguments | Stack input | Stack output  | Description |
|-----------|-----------|-------------|---------------|-------------|
| `noop`    |     -     |      -      |       -       | Does nothing.

### Stack

Stack elements are zero-indexed, i.e. `0` is the top element, `1` is then next one.

| Operation | Arguments | Stack input | Stack output  | Description |
|-----------|-----------|-------------|---------------|-------------|
| `push`    |   1 VLQ   |      -      |       1       | Pushes encoded constant value.
| `pop`     |     -     |      1      |       -       | Consumes one element.
| `copy`    |   1 VLQ   |      -      |       1       | Copies n-th element and pushes onto the stack.
| `swap`    |   1 VLQ   |      -      |       1       | Swaps n-th and top elements in the stack.

### Arithmetic

| Operation | Code | Arguments | Stack input | Stack output  | Description |
|-----------|------|-----------|-------------|---------------|-------------|
| `add`     | `08` |     -     |     2     |     1     | Consumes two elements, pushes their sum.
| `sub`     | `09` |     -     |     2     |     1     | Consumes two elements, pushes their difference (subtracts the second element from the first one).
| `mul`     | `0A` |     -     |     2     |     1     | Consumes two elements, pushes their product.
