# First circuit

## Zinc installation

To start using the Zinc framework, do the following:

1. Download its [binaries](https://github.com/matter-labs/zinc/releases) for your OS and architecture.
2. Add the folder with the binaries to `PATH`
3. Use the binaries via your favorite terminal

The Zinc framework consists of three tools:

- `zargo` circuit manager
- `znc` Zinc compiler
- `zvm` Zinc virtual machine

`zargo` can use the compiler and virtual machine through its interface,
so you will only need `zargo` to work with your circuits.

For more information on `zargo`, check out this [chapter](../10-zargo-circuit-manager/00-overview.md).

Let's now move on to writing 'Hello, World!' in Zinc!

## The Visual Studio Code extension

There is a syntax highlighting extension for Zinc called `Zinc Syntax Highligthing`.
The IDE should recommend installing it once you have opened a Zinc file!

## Creating the circuit

Let's create our first circuit, which will be able to prove knowledge of
some `sha256` hash preimage:

```
zargo new --circuit preimage
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
the project root. There is also a file called `witness.json` in the
`build` directory, which is used to provide the secret witness data to the circuit.

### Running the circuit

Before you run the circuit, open the `data/witness.json` file with
your favorite editor and fill it with some meaningful values.

Now, execute `zargo run > data/public-data.json` to run the circuit and
write the resulting public data to a file.

> There is a useful tool called `jq`. You may use it together with `zargo run`
> to highlight, edit, filter the output data before writing it to the file:
> `zargo run | jq > data/public-data.json`.
> 
> For more information on `jq`, visit the [official manual](https://stedolan.github.io/jq/manual/).

### Trusted setup

To be able to verify proofs, you must create a pair of keys for the prover and
the verifier.

To generate a new pair of proving and verifying keys, use this command:

```bash
zargo setup
```

### Generating a proof

To generate a proof, provide the witness and public data to the Zinc VM with
the following command:

```bash
zargo prove > proof.txt
```

This will also write the program's output to `data/public-data.json` which is later
used by the verifier.

### Verifying a proof

Before verifying a proof, make sure the prover and verifier use the same
version of the Zinc framework.
Some versions may be compatible, but it is to be decided yet.

To verify a proof, pass it to the Zinc VM with the same public data you used to
generated it, and the verification key:

```bash
zargo verify < proof.txt
```

Congratulations! You have developed your first circuit and verified your first
Zero-Knowledge Proof!

Now you may proceed to implementing the [more complex example](03-more-complex-example.md)
after reading about the [built-in functions](01-builtin-functions.md) and [standard library](02-standard-library.md).
