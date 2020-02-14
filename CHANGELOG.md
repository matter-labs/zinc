# The Zinc changelog

## Version 0.1.2

### Language

- the structure literal does not require the `struct` keyword anymore
- match scrutinee expression now can only be a single identifier (will be fixed soon)
- `dbg!(...)` string interpolation, e.g. `dbg!("{} + {} = {}", 2, 2, 4)`;
- `assert!(...)` now accepts an optional string message as the 2nd argument
- operators `/`, `%`, `>=`, `>`, `<=`, `<` are temporarily forbidden for the type `field`

### Zargo

- the 'run' command now builds the circuit before running
- added the 'proof-check` command, which executes the sequence 'build + setup + proof + verify'
- circuit data (keys, inputs, outputs) moved from `build` to `data` folder

### Compiler
- fixed all the known bugs
- improved some error messages

### Virtual machine
- fixed all the known bugs
- pretty error reporting with file, line, and column
- removed the redundant 'field' and 'value' keys from the structure type in input JSON templates

### Overall
- full integration test coverage
- improved logging
