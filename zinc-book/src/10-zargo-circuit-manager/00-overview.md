# Zargo circuit manager

`Zargo` is a circuit managing tool, which can create, build, and use circuits
to generate and verify proofs.

## Commands overview

All the commands have default values, so you may omit them in normal circumstances.
See `zargo --help` for more detail.

### `new`

Creates a new project directory with `Zargo.toml` manifest file and `src/main.zn`
circuit entry point module.

### `init`

Initializes a new project in an existing directory, creates missing files.

### `build`

Builds the circuit. The build consists of:
- the bytecode file
- secret input JSON template
- public data JSON template

### `clean`

Removes the build directory.

### `run`

Build and runs the circuit on the Zinc VM, writes the result to the terminal.

### `setup`

Generates parameters for the prover using the circuit bytecode.

### `prove`

Generates the proof using the circuit bytecode, parameters generated with `setup`,
and provided public data.

### `verify`

Verifies the proof using the circuit bytecode, parameters generated with `setup`,
proof generated with `prove`, and provided public data.

### `proof-check`

Executes the full cycle of proof verification, that is, performs
`run` + `setup` + `prove` + `verify`. Mostly for testing purposes.

## Workflow example

### Short

```bash
# create a new circuit called 'zircuit'
zargo new --type circuit zircuit
cd zircuit/

# write some code

# run the full verification cycle
zargo proof-check
```

### Long

```bash
# create a new circuit called 'zircuit'
zargo new --type circuit zircuit
cd zircuit/

# write some code

# build the circuit
zargo build

# run the circuit and print the result
zargo run

# generate the prover parameters
zargo setup

# edit the 'build/witness.json' and 'build/public-data.json' files

# generate the proof
zargo prove

# verify the proof
zargo verify
```

## Manifest file

A Zinc circuit is described with the manifest file `Zargo.toml` with the
following structure:

```toml
[circuit]
name = "test"
version = "0.1.0"
```
