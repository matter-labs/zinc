# First circuit

## Cargo installation

To install Zinc into your system, you must first install the Rust package manager `cargo`.

On Linux or Mac OS, this simple command will work:

`curl https://sh.rustup.rs -sSf | sh`

If you are using Windows, download the [installer](https://win.rustup.rs/).

For more information on `cargo` and its installation, see
[the Cargo book](https://doc.rust-lang.org/cargo/getting-started/installation.html).

## Zinc installation

Once you have `cargo` installed into your system, you can download the Zinc
binaries from its repository:

`cargo install zinc`

This command will install the following binaries into your `PATH`:

- `zargo` circuit manager
- `znc` Zinc compiler
- `zinc` Zinc virtual machine

`zargo` can use the compiler and virtual machine through its interface,
so you will only need `zargo` to work with your circuits.

For more information on `zargo`, check out this [chapter](../09-zargo-circuit-manager/00-overview.md).

Let's now move on to writing 'Hello, World!' in Zinc!

## Creating the circuit

Let's create our first circuit, which will be able to prove knowledge of
some `sha256` hash preimage:

```
zargo new preimage
cd preimage
```

The command above will create a directory with `Zargo.toml` manifest and the `src/`
folder with an entry point module `main.zn`.

Let's replace the `main.zn` contents with the following code:

```rust,no_run,noplaypen
use std::sha256;

fn main(preimage: [bool; 256]) -> [bool; 256] {
    sha256(preimage)
}
```

## Building the circuit

Now, you need to compile the circuit into Zinc bytecode:

`zargo build`

The command above will write the bytecode to the `build` directory located in
the project root. There is also a file called `witness.json` in the
`build` directory, which is used to provide the secret witness data to the circuit.

## Trusted setup

To be able to verify proofs, you must create a pair of keys for the prover and
the verifier.

To generate a new pair of proving and verifying keys, use this command:

```bash
zargo setup
```

## Generating a proof

Before generating a proof, open the `build/witness.json` file with
your favorite editor and fill it with some meaningful values.

To generate a proof, provide the witness and public data to the Zinc VM with
the following command:

```bash
zargo prove > proof.txt
```

This will also write the program's output to `build/pubdata.json` which is later
used by the verifier.

## Verifying a proof

Before verifying a proof, make sure that the prover and verifier use the same
version of the Zinc framework.
Some versions may be compatible, but it is to be decided yet.

To verify a proof, pass it to the Zinc VM with the same public data you used to
generated it, and the verification key:

```bash
zargo verify < proof.txt
```

Congratulations! You have developed your first circuit and verified your first
Zero-Knowledge Proof!

Feel free to proceed to the next chapters to know more about the Zinc framework!
