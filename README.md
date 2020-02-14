# The Zinc framework

## Workflow example

At first, you should install the following binaries into your `PATH`:
- `zargo` - the circuit management tool
- `znc` - the Zinc compiler
- `zvm` - the Zinc virtual machine

Then, follow the example to create and use your first circuit:

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build the circuit
zargo build

# fill the input template JSON usually located at ./data/input.json with values

# runs the circuit to check it without input data
zargo run

# generate the prover parameters
zargo setup

# generate the proof
zargo prove > './data/proof.txt'

# verify the proof
zargo verify < './data/proof.txt'
```

**OR**

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build & run & setup & prove & verify
zargo proof-check
```

## Roadmap

### Version 0.1.2

- [x] primitive types 
- [x] witness and public data types exported as JSON
- [x] var declarations
- [x] operators on primitive types
- [x] `assert!()`
- [x] comments
- [x] `dbg!()`
- [x] syntax highlighting for Visual Studio Code
- [x] mutability
- [x] conditionals
- [x] `for` loops (without `while`)
- [x] standard library (from bellman)
- [x] `for` loops with `while`
- [x] arrays
- [x] tuples
- [x] type aliases
- [x] structures
- [x] functions
- [x] modules and imports
- [x] C-like `enum`
- [x] `match`
- [x] standard library

### Later

- [ ] conditional optimization
- [ ] `Option<>`
- [ ] `Result<>`, ? operator
- [ ] move() / copy()
- [ ] `unsafe_unchecked`
- [ ] testing framework with coverage metrics
- [ ] strings?
- [ ] interfaces?
- [ ] `unsafe_rust`?
- [ ] formal verification
