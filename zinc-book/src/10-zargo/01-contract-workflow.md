# Contract workflow

This code snippet describes the workflow of creating, building, publishing a
smart contract and calling its methods.

```bash,no_run,noplaypen
# create a new contract called 'swap'
zargo new --type contract swap
cd swap/

# write some code

# rebuild, publish the contract, and get its address
zargo publish --instance default --network rinkeby

# query the newly created contract storage
zargo query --address <address>

# call some contract method
zargo call --method exchange --address <address>
```

## Manifest file

A Zinc smart contract is described in the manifest file `Zargo.toml` with the
following structure:

```toml,no_run,noplaypen
[project]
name = 'test'
type = 'contract'
version = '0.1.0'
```
