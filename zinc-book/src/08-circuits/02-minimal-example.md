# Minimal example

## Creating a circuit

Let's create our first circuit, which will be able to prove knowledge of
some `sha256` hash preimage:

```bash,no_run,noplaypen
zargo new --type circuit preimage
cd preimage
```

The command above will create a directory with `Zargo.toml` manifest and the `src/`
folder with an entry point module `main.zn`.

Let's replace the `main.zn` contents with the following code:

```rust,no_run,noplaypen
use std::crypto::sha256;
use std::convert::to_bits;
use std::array::pad;

const FIELD_SIZE: u64 = 254;
const FIELD_SIZE_PADDED: u64 = FIELD_SIZE + 2 as u64;
const SHA256_HASH_SIZE: u64 = 256;

fn main(preimage: field) -> [bool; SHA256_HASH_SIZE] {
    let preimage_bits: [bool; FIELD_SIZE] = to_bits(preimage);
    let preimage_bits_padded: [bool; FIELD_SIZE_PADDED] = pad(preimage_bits, 256, false);
    sha256(preimage_bits_padded)
}
```

## All-in-one command

When you have finished writing the code, run `zargo proof-check`. This command
will build and run the circuit, generate keys for the trusted setup, generate a proof
and verify it.

## Step by step

Let's get through each step of the command above manually to better understand
what is under the hood. Before you start, run `zargo clean` to remove all the
build artifacts.

### Building the circuit

Now, you need to compile the circuit into Zinc bytecode:

`zargo build`

The command above will write the bytecode to the `build` directory located in
the project root. There is also a file called `input.json` in the
`data` directory, which is used to provide the secret witness data to the circuit.

### Running the circuit

Before you run the circuit, open the `./data/input.json` file with
your favorite editor and fill it with some meaningful values.

Now, execute `zargo run > ./data/output.json` to run the circuit and
write the resulting public data to a file.

> There is a useful tool called `jq`. You may use it together with `zargo run`
> to highlight, edit, filter the output data before writing it to the file:
> `zargo run | jq > ./data/output.json`.
> 
> For more information on `jq`, visit the [official manual](https://stedolan.github.io/jq/manual/).

### Trusted setup

To be able to verify proofs, you must create a pair of keys for the prover and
the verifier.

To generate a new pair of proving and verifying keys, use this command:

```bash,no_run,noplaypen
zargo setup
```

### Generating a proof

To generate a proof, provide the witness and public data to the Zinc VM with
the following command:

```bash,no_run,noplaypen
zargo prove > proof.txt
```

### Verifying a proof

Before verifying a proof, make sure the prover and verifier use the same
version of the Zinc framework.

To verify a proof, pass it to the Zinc VM with the same public data you used to
generate it and the verification key:

```bash,no_run,noplaypen
zargo verify < proof.txt
```

Congratulations! You have developed your first circuit and verified your first
Zero-Knowledge Proof!

Now you may proceed to implementing the [more complex example](./03-merkle-tree.md).