# Compiler command line interface

## Calling compiler

```sh
jab {file_name or working_dir}
```

## Output

- list of inputs (json)
- list of outputs (json)
- witness/circuit generator (rust code)

## Cost profiler

The cost profiler must output the program statement by statement in the canonical form, adding comments of the number of constraints.

Control statements with blocks must indicate total cost of the block enclosed in `{ ... }`. 

`if` and `for` statements must show the cost of the condition and the total cost separately.

```rust
let a = 5; let b = a*a;

if a >= b { a = 1; } else { a = 2; }
```

must convert to:

```rust
let a: uint8 = 5; // 0
let b = a*a; // 1

if a >= b // 16 {24019}
{ // {3}
    a = 1; // 3
} else { // {24000}
    a = sha256(b); // 24000
}
```
