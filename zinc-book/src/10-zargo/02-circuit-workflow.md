# Circuit workflow

### Short

The short example includes the `proof-check` command, which executes a full
application lifecycle with default data.

```bash,no_run,noplaypen
# create a new circuit called 'zircuit'
zargo new --type circuit zircuit
cd zircuit/

# write some code

# run the full verification cycle
zargo proof-check
```

### Full

The full workflow example allows you to go through the application lifecycle
step by step and see all its intrincics.

```bash,no_run,noplaypen
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

A Zinc circuit is described in the manifest file `Zargo.toml` with the
following structure:

```toml,no_run,noplaypen
[project]
name = "test"
type = "circuit"
version = "0.1.0"
```
