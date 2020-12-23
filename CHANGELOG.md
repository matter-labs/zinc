# The Zinc changelog

## Version 0.2.1 (2020-12-24)

#### Language

- implemented the Rust-like dependency system
- added the library project type
- allowed to create and use other smart contract instances using the dependency system
- extended the attribute syntax to set the `zksync::msg` variable in unit tests
- turned the `zksync::transfer` function into a built-in contract method

#### Compiler

- fixed the bug with invalid scope for contract fields and methods
- fixed the bug where enum variants were not treated as constants
- fixed the bug where contract storage fields could be addressed without an instance
- fixed the bug where structures and contracts could be initialized with fewer fields than needed
- performed an inventory of compiler errors and created an error reference

#### Virtual machine

- prevented contract storage and zkSync side effects in false condition branches
- fixed circuit unit tests
- replaced the `Exit` instruction with `Return`
- disabled the proof verification tools, as they are temporarily unsupported

#### Zargo

- initial deposits for contract publishing are not required anymore
- fixed the segmentation fault during HTTP requests on Linux

#### Book

- added the smart contract troubleshooting chapter
- added the dependency system chapter

#### Source code

- merged
    - `zinc_build` and `zinc_zksync` into `zinc_types`,
    - `zinc_source` and `zinc_manifest` into `zinc_project`

## Version 0.2.0 (2020-10-28)

#### Language

*General*

- implemented the module system with access to the root and parent modules
- allowed declaring constants, types, functions, and modules in arbitrary order
- added aliases to the `use` import statement
- implemented unit-testing functionality
- renamed the `assert!` function to `require`

*Object-oriented*

- added methods with by-value `self` instance parameter
- added the syntax sugar for calling methods via the dot `.` operator

*Compile-time*

- extended constant expressions with arrays, tuples, structures, blocks, conditionals, and matches
- implemented primitive constant functions

*Expressions*

- fixed the issue where operations on enums could result into an invalid variant
- allowed bitwise operations on non-constant and witness values
- forbidden bitwise operations on signed integers and fields
- implemented the type inference for integer literals in expressions
- added the exponent notation of integer literals
- implemented declaration of multiple variables via tuple destructuring
- removed the mandatory semicolon after block, conditional, and match expressions

#### Compiler

- fixed the bug where namespace items where resolved from outside the namespace
- fixed the bug where the type of `match` enumeration variant path pattern used to be unchecked
- fixed the bug where a structure could be initialized without fields
- implemented the per-entry dead code elimination optimization for unreachable functions
- implemented the short-circuiting boolean expressions evaluation
- improved some error messages

#### Virtual machine

- added the overflow checking and `enum` validation for input JSON files
- allowed witness array indexes without constraints
- improved some error messages

#### Zargo

- generalized for managing both circuit and smart contract projects
- improved some commands and default values to decrease redundancy
- added some commands for publishing and working with smart contracts
- taught Zargo to mimic Cargo more accurately

#### Source code

- added doc comments for the entire project using `missing_docs_*` lints
- moved some shared data to crates:
    - `zinc_logger`
    - `zinc_const`
    - `zinc_math`
    - `zinc_manifest`
    - `zinc_source`
    - `zinc_types`
    - `zinc_types`

#### Overall

- implemented basic smart contracts
- developed the Zandbox application for publishing and running smart contracts

## Version 0.1.5 (2020-04-07)

#### Language

- forbidden the division operator `/`, but implemented `std::ff::invert` for `field` inversion
- allowed casting to types with lesser bitlength (runtime error on overflow)
- added the bitwise operators `|`, `|=`, `^`, `^=`, `&`, `&=`, `<<`, `<<=`, `>>`, `>>=`, `~` (constant expressions only)
- added the binary (e.g. `0b101010`) and octal (e.g. `0o52`) integer literals
- implemented match exhaustiveness checking without the binding or wildcard pattern
- removed `static` statements for global variables (use `const` instead)
- limited `match` scrutinee expression to boolean and integer only, since it is impossible to destructure complex types for now
- reserved some keywords (see [Appendix C](https://zinc.matterlabs.dev/appendix/C-keywords.html) of the Zinc book)

#### Compiler

- fixed the bug with `!` while passing a non-builtin function call as a built-in one's argument
- fixed the bug with duplicate match expression branches

#### Overall

- added a wrapper directory to the release archives to prevent tar- and zip-bombs

## Version 0.1.4 (2020-03-05)

#### Language

- added the Schnorr signature verification to the standard library
- made enumerations strongly typed, not just groups of constants
- match scrutinee can be any expression again (including structure literals)
- implemented automatic loop bounds range type upcasting
- implemented arithmetic assignment operators (`+=`, `-=`, `*=`, `/=`, `%=`)
- allowed constant expressions as array sizes (both types and literals)
- field division (i.e. multiplication by inverted value)
- field comparison (treated as unsigned numbers)

#### Compiler

- implemented advanced errors with Rust-like formatting, hints, location pointers
- added constant overflow checking at compile-time
- the constant expression Euclidean division and remainder now work just like in VM

#### Virtual machine

- fixed 'unconstrained' variables in hash functions
- fixed constraint generation when the same variable is encountered multiple times in the same expression
- fixed some type errors
- optimized constraint generation for deterministic expressions

#### Overall

- added the Schnorr signature tool

## Version 0.1.3 (2020-02-17)

#### Compiler

- fixed the compile error with a comment at the end of a file
- added an empty statement to allow optional semicolons

## Version 0.1.2 (2020-02-14)

#### Language

- the structure literal does not require the `struct` keyword anymore
- `dbg!(...)` string interpolation, e.g. `dbg!("{} + {} = {}", 2, 2, 4)`;
- `assert!(...)` now accepts an optional string message as the 2nd argument
- match scrutinee expression now can only be a single identifier (will be fixed soon)
- operators `/`, `%`, `>=`, `>`, `<=`, `<` are temporarily forbidden for the type `field`

#### Zargo

- the 'run' command now builds the circuit before running
- added the 'proof-check` command, which executes the sequence 'build + setup + proof + verify'
- circuit data (keys, inputs, outputs) moved from `build` to `data` folder

#### Compiler

- fixed many boundaries of integer types
- fixed the loop range overflow in some edge values of integer types
- fixed the invalid operand order bug in some cases
- fixed conflicting namespaces for functions and types with the same name
- improved some error messages

#### Virtual machine

- fixed `pedersen` hash
- fixed unsigned integers division
- fixed the `while` condition
- fixed the function argument order in some `std` functions
- made the `std::convert::from_bits_signed` result two-complement
- pretty error reporting with file, line, and column
- improved some error messages
- removed the redundant 'field' and 'value' keys from the structure type in input JSON templates

#### Overall

- full integration test coverage
- improved logging

## Version 0.1.1 (2020-02-08)

*Internal dogfooding/testing release*

## Version 0.1.0 (2020-01-31)

*Initial release*
