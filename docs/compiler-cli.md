# Compiler command line interface

## The interface description

```
jabc 0.1.0
hedgar2017 <hedgar2017@gmail.com>
The Jabberwocky language compiler

USAGE:
    jabc.exe [FLAGS] --input <INPUT> --output <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -m, --meta       Generates meta info
    -p, --profile    Runs the profiler and prints cost information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>      Specifies the input *.jab file name
    -o, --output <OUTPUT>    Specifies the output *.rs file name
```

## Meta info

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

## Cost profiler output

The cost profiler must print number of constraints for each line in the following `json` format:

```json
{
    "file": "filename.jab",
    "md5":  "md5 hash of the file",
    "constraints": {
        "1": 2,
        "2": 0,
        "3": 1,
        "4": {"inline": 4, "block": 25},
    }
}
```

Each line must sum up constraints in all statements that **begin** in this line.

If a line contains the beginning of a block enclosed in `{ ... }`, the costs must include the total cost of the block in curly brackets:

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
    "1": {"inline": 3, "block": 4},
    "2": 1,
    "3": {"inline": 0, "block": 2},
    "4": 2
}
```

This information will be used to visualize the cost with IDE plugins.
