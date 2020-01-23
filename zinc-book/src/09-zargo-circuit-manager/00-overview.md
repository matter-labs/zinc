# Zargo circuit manager

`Zargo` is a circuit managing tool, which can create, build, and use circuits
to generate and verify proofs.

## Commands overview

All the commands have default values, so you may omit them in usual circumstances.
See the tool `--help` for more detail.

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

### `exec`

Executes the circuit on the Zinc VM, writes the circuit result to a JSON file.

### `setup`

Generates parameters for the prover using the circuit bytecode.

### `prove`

Generates the proof using the circuit bytecode, parameters generated with `setup`,
and provided public data.

### `verify`

Verifies the proof using the circuit bytecode, parameters generated with `setup`,
proof generated with `prove`, and provided public data.

## Workflow example

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build the circuit
zargo build

# fill the input template JSON usually located at ./build/input.json with values

# execute the circuit to check it without input data
zargo exec

# generate the prover parameters
zargo setup

# generate the proof
zargo prove

# verify the proof
zargo verify
```

## Manifest file

Every Zinc circuit is described with a manifest file `Zargo.toml` with the
following structure:

```toml
[circuit]
name = "test"
version = "0.1.0"
```
