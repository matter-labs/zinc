# Zargo package manager

`Zargo` is a project managing tool, which can create and build projects,
publish smart contracts and call their methods.

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
- input JSON template
- output JSON template

### `clean`

Removes the build directory.

### `run`

Build and runs the application on the Zinc VM, writes the result to the terminal.

### `test`

Runs the application unit tests.

## Smart contract commands

### `publish`

Publishes the smart contract to the Zandbox server on the specified network.

### `query`

Queries a smart contract storage or calls an immutable method.

### `call`

Calls a mutable smart contract method, that is, one modifying its storage and
making operations with tokens and balances.

### `upload`

Uploads the project to the Zandbox server on the specified network.

### `download`

Downloads the project from the Zandbox server on the specified network.
