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

# fill the input template JSON usually located at ./build/input.json with values

# runs the circuit to check it without input data
zargo run

# generate the prover parameters
zargo setup

# generate the proof
zargo prove

# verify the proof
zargo verify
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

- [x] `for` loops with `while`
- [x] arrays
- [x] tuples
- [x] type aliases
- [x] structures
- [x] code generation of all of the above

### Version 0.5

- [x] functions
- [x] modules and imports
- [x] C-like `enum`
- [x] `match`

### Version 0.6

- [ ] conditional optimization
- [x] std lib
- [ ] `Option<>`

### Version 0.7

- [ ] `Result<>`, ? operator
- [ ] simple references (more like aliases)
- [ ] move() / copy()

### Later

- [ ] `unsafe_unchecked`
- [ ] testing framework with coverage metrics
- [ ] bytes and strings
- [ ] interfaces?
- [ ] `unsafe_rust`?
- [ ] formal verification
