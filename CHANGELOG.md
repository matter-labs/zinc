# The Zinc changelog

## Version 0.1.3 (2020-02-17)

### Compiler

- fix the compile error with a comment at the end of a file
- add an empty statement to allow optional semicolons

## Version 0.1.2 (2020-02-14)

### Language

- the structure literal does not require the `struct` keyword anymore
- `dbg!(...)` string interpolation, e.g. `dbg!("{} + {} = {}", 2, 2, 4)`;
- `assert!(...)` now accepts an optional string message as the 2nd argument
- match scrutinee expression now can only be a single identifier (will be fixed soon)
- operators `/`, `%`, `>=`, `>`, `<=`, `<` are temporarily forbidden for the type `field`

### Zargo

- the 'run' command now builds the circuit before running
- added the 'proof-check` command, which executes the sequence 'build + setup + proof + verify'
- circuit data (keys, inputs, outputs) moved from `build` to `data` folder

### Compiler

- fixed many boundaries of integer types
- fixed the loop range overflow in some edge values of integer types
- fixed the invalid operand order bug in some cases
- fixed conflicting namespaces for functions and types with the same name
- improved some error messages

### Virtual machine

- fixed `pedersen` hash
- fixed unsigned integers division
- fixed the `while` condition
- fixed the function argument order in some `std` functions
- made the `std::convert::from_bits_signed` result two-complement
- pretty error reporting with file, line, and column
- improved some error messages
- removed the redundant 'field' and 'value' keys from the structure type in input JSON templates

### Overall

- full integration test coverage
- improved logging

## Version 0.1.1 (2020-02-08)

*Internal dogfooding/testing release*

## Version 0.1.0 (2020-01-31)

*Initial release*
