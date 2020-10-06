# Zargo package manager

`Zargo` is a project managing tool, which can create and build projects,
generate and verify proofs, publish smart contracts and call their methods.

## General commands

All the commands have default values, so you may omit them in normal circumstances.
See `zargo --help` for more detail.

### `new`

Creates a new project directory with `Zargo.toml` manifest file and `src/main.zn`
application entry point module.

### `init`

Initializes a new project in an existing directory, creates missing files.

### `build`

Builds the project. The build consists of:
- the bytecode file
- witness input JSON template
- public data JSON template

### `clean`

Removes the build directory.

### `run`

Build and runs the application on the Zinc VM, writes the result to the terminal.

### `test`

Runs the application unit tests.

### `setup`

Generates parameters for the prover using the application bytecode.

### `prove`

Generates the proof using the application bytecode, parameters generated with `setup`,
and provided public data.

### `verify`

Verifies the proof using the application bytecode, parameters generated with `setup`,
proof generated with `prove`, and provided public data.

### `proof-check`

Executes the full cycle of proof verification, that is, performs
`run` + `setup` + `prove` + `verify`. Mostly for testing purposes.

## Smart contract commands

### `publish`

Publishes the smart contract to the Zandbox server on the specified network.

### `query`

Queries a smart contract storage or calls an immutable method.

### `call`

Calls a mutable smart contract method, that is, one modifying its storage and
making operations with tokens and balances.
