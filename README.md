# The Jabberwocky translator

## Roadmap

### Version 0.1

- [x] primitive types 
- [x] input and witness (exported as JSON output)
- [x] var declarations
- [x] operators on primitive types
- [x] `require()`
- [x] comments
- [x] `debug!()`
- [x] interpreter

### Version 0.2

- [x] syntax highlighting for Visual Studio Code
- [x] mutability
- [x] conditionals
- [x] `for` loops (without `while`)

### Version 0.3

- [x] standard library (from bellman)

### Version 0.4

- [x] type aliases
- [x] `for` loops with `while`
- [x] arrays
- [x] tuples
- [x] structures
- [x] code generation of all of the above

### Version 0.5

- [ ] functions
- [ ] modules and imports

### Later

- [ ] `enum`
- [ ] `match`
- [ ] `unsafe_unchecked`

### Much later

- [ ] testing framework with coverage metrics
- [ ] `Option<>`, `Result<>`, etc
- [ ] bytes and strings
- [ ] interfaces?
- [ ] `unsafe_rust`?
- [ ] conditional optimization
- [ ] formal verification

## Transpiler output

### Meta info

```json
{
    "inputs": [
        {
          "identifier": { "name": "a" },
          "type": {"name": "uint", "bitlength": 8}
        }
    ],
    "witnesses": [
        {
          "identifier": { "name": "b" },
          "type": {"name": "int", "bitlength": 248}
        }
    ]
}
```

### Cost profiling

The cost profiler prints number of constraints for each line:

```json
{
    "file": "filename.jab",
    "md5":  "000011112222333344445555666677778888",
    "constraints": {
        "1": 2,
        "2": 0,
        "3": 1,
        "4": {"inline": 4, "block": 25},
    }
}
```

Each line must sum up constraints in all statements that begin in this line.

If a line contains the beginning of a block enclosed in `{ ... }`, the costs
must include the total cost of the block in curly brackets:

```rust
1: if a == b { // 3 constraints
2:     t = a * b; // 1 constraints
3: } else {
4:     t = a * b * c; // 2 constraint
5: }
```

=>

```json
"constraints": {
    "1": { "inline": 3, "block": 4 },
    "2": 1,
    "3": { "inline": 0, "block": 2 },
    "4": 2
}
```

This information will be used to visualize the cost with IDE plugins.
