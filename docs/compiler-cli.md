# Compiler command line interface

## Calling compiler

```sh
jab [-p {filename_profiler_output}] [-o {output.rs}] [-m {filename_meta}] {file_name}
```

Options:

- `-p`, `--profile`: run profiler and print cost information
- `-o`, `--output`: specify output .rs file name
- `-m`, `--meta`: generate meta info

## Meta info

```json
{
   "inputs":[
      {
         "identifier":"a",
         "type":{
            "name": "uint",
            "size": 8
         }
      },
      {
         "identifier":"b",
         "type":"field"
      },
      {
         "identifier":"c",
         "type":"bool"
      }
   ],
   "witness":[
      {
         "identifier":"a",
         "type":{
            "name": "uint",
            "size": 8
         }
      },
      {
         "identifier":"b",
         "type":"field"
      },
      {
         "identifier":"c",
         "type":"bool"
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
2:      t = a * b; // 1 constraints
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