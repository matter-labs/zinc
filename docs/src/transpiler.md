# Transpiler

## Interface

```
zrsc 0.4.0
The ZRust language compiler

USAGE:
    transpiler.exe [FLAGS] --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -m, --meta       Generates meta info
    -p, --profile    Runs the profiler and prints cost information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Specifies the input *.zrs file name
    -o, --output <output>    Specifies the output *.rs file name
```

## Output

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
    "file": "filename.zrs",
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
