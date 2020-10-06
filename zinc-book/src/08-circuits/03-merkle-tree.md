# The Merkle tree

In this chapter, we will implement a circuit able to validate the Merkle tree
root hash.

At this stage of reading the book, you may be unfamiliar with some language
concepts. So, if you struggle to understand some examples, you are welcome to
read the rest of the book first, and then come back.

Our circuit will accept the tree node path, address, and the balance available
as the secret witness data. The public data will be the Merkle tree root hash.

## Creating a new project

Let's create a new circuit called `merkle-proof`:

```bash,no_run,noplaypen
zargo new --type circuit merkle-proof
cd merkle-proof
```

Now, you can open the project in your favorite IDE and go to `src/main.zn`,
where we are going to start writing the circuit code.

## Defining types

Let's start by defining the secret witness data arguments and the public data
return type.

```rust,no_run,noplaypen
struct PublicInput {
    root_hash: [bool; 256],
}

fn main(
    address: [bool; 10], // the node address in the merkle tree
    balance: field, // the balance stored in the node
    merkle_path: [[bool; 256]; 10] // the hash path to the node
) -> PublicInput {
    // ...
}
```

As you can see, some complex types are used in several places of our code, so
it is very convenient to create an alias for such type.

```rust,no_run,noplaypen
type Sha256Digest = [bool; 256];
```

## Creating functions

Now, we will write a function to calculate the `sha256` hash of
our balance. We need it to verify the balance stored within the leaf node at our
Merkle tree path.

```rust,no_run,noplaypen
fn balance_hash(balance: field) -> Sha256Digest {
    let bits = std::convert::to_bits(balance); // [bool; 254]
    let bits_padded = std::array::pad(bits, 256, false); // [bool; 256]
    std::crypto::sha256(bits_padded) // [bool; 256] a.k.a. Sha256Digest
}
```

The function accepts `balance` we passed as secret witness data, converts it
into a bit array of length 254 (elliptic curve field length), and pads the
array with 2 extra zero bits, since we are going to pass a 256-bit array to the
`sha256` function.

We have also used here three functions from the [standard library](../appendix/E-standard-library.md)
from three different modules. Paths like `std::crypto::sha256` might seem a
bit verbose, but we will solve this problem later.

At this stage, this is how our code looks like:

```rust,no_run,noplaypen
type Sha256Digest = [bool; 256];

struct PublicInput {
    root_hash: Sha256Digest,
}

fn balance_hash(balance: field) -> Sha256Digest {
    let bits = std::convert::to_bits(balance); // [bool; 254]
    let bits_padded = std::array::pad(bits, 256, false); // [bool; 256]
    std::crypto::sha256(bits_padded) // [bool; 256] a.k.a. Sha256Digest
}

fn main(
    address: [bool; 10], // the node address in the merkle tree
    balance: field, // the balance stored in the node
    merkle_path: [Sha256Digest; 10] // the hash path to the node
) -> PublicInput {
    let leaf_hash = balance_hash(balance);

    // ...
}
```

Now, we need a function to calculate a tree node hash:

```rust,no_run,noplaypen
fn merkle_node_hash(left: Sha256Digest, right: Sha256Digest) -> Sha256Digest {
    let mut data = [false; 512]; // [bool; 512]

    // Casting to u16 is needed to make the range types equal,
    // since 0 will be inferred as u8, and 256 - as u16.
    for i in 0 as u16..256 {
        data[i] = left[i];
        data[256 + i] = right[i];
    }

    std::crypto::sha256(data) // [bool; 256] a.k.a. Sha256Digest
}
```

The Zinc standard library does not support array concatenation yet, so for now,
we will do it by hand, allocating an array to contain two leaves node digests,
then put the digests together and hash them with `std::crypto::sha256`.

Finally, let's define a function to calculate the hash of the whole tree:

```rust,no_run,noplaypen
fn restore_root_hash(
    leaf_hash: Sha256Digest,
    address: [bool; 10],
    merkle_path: [Sha256Digest; 10],
) -> Sha256Digest
{
    let mut current = leaf_hash; // Sha256Digest

    // Traverse the tree from the left node to the root node
    for i in 0..10 {
        // Multiple variables binding is not supported yet,
        // so we going to store leaves as an array of two digests.
        // If address[i] is 0, we are in the left node, otherwise,
        // we are in the right node.
        let left_and_right = if address[i] {
            [current, merkle_path[i]] // [Sha256Digest; 2]
        } else {
            [merkle_path[i], current] // [Sha256Digest; 2]
        };

        // remember the current node hash
        current = merkle_node_hash(left_and_right[0], left_and_right[1]);
    }

    // return the root node hash
    current
}
```

Congratulations! Now we have a working circuit able to verify the Merkle proof!

```rust,no_run,noplaypen
// main.zn

type Sha256Digest = [bool; 256];

fn balance_hash(balance: field) -> Sha256Digest {
    let bits = std::convert::to_bits(balance); // [bool; 254]
    let bits_padded = std::array::pad(bits, 256, false); // [bool; 256]
    std::crypto::sha256(bits_padded) // [bool; 256] a.k.a. Sha256Digest
}

fn merkle_node_hash(left: Sha256Digest, right: Sha256Digest) -> Sha256Digest {
    let mut data = [false; 512]; // [bool; 512]

    for i in 0..256 {
        data[i] = left[i];
        data[256 + i] = right[i];
    }

    std::crypto::sha256(data) // [bool; 256] a.k.a. Sha256Digest
}

fn restore_root_hash(
    leaf_hash: Sha256Digest,
    address: [bool; 10],
    merkle_path: [Sha256Digest; 10],
) -> Sha256Digest
{
    let mut current = leaf_hash; // Sha256Digest

    // Traverse the tree from the left node to the root node
    for i in 0..10 {
        // Multiple variables binding is not supported yet,
        // so we going to store leaves as a tuple of two digests.
        // If address[i] is 0, we are in the left node, otherwise,
        // we are in the right node.
        let left_and_right = if address[i] {
            (current, merkle_path[i]) // (Sha256Digest, Sha256Digest)
        } else {
            (merkle_path[i], current) // (Sha256Digest, Sha256Digest)
        };

        // remember the current node hash
        current = merkle_node_hash(left_and_right.0, left_and_right.1);
    }

    // return the root node hash
    current
}

struct PublicInput {
    root_hash: Sha256Digest,
}

fn main(
    address: [bool; 10],
    balance: field,
    merkle_path: [Sha256Digest; 10]
) -> PublicInput {
    let leaf_hash = balance_hash(balance);

    let root_hash = restore_root_hash(
        leaf_hash,
        address,
        merkle_path,
    );

    PublicInput {
        root_hash: root_hash,
    }
}
```

## Defining a module

Our `main.zn` module has got a little overpopulated by now, so let's move our
functions to another one called `merkle`. At first, create a file called `merkle.zn`
in the `src` directory besides `main.zn`. Then, move everything above the
`PublicInput` definition to that file. Our `main.zn` will now look like this:

```rust,no_run,noplaypen
struct PublicInput {
    root_hash: Sha256Digest, // undeclared `Sha256Digest`
}

fn main(
    address: [bool; 10],
    balance: field,
    merkle_path: [Sha256Digest; 10] // undeclared `Sha256Digest`
) -> PublicInput {
    let leaf_hash = balance_hash(balance); // undeclared `balance_hash`

    let root_hash = restore_root_hash( // undeclared `restore_root_hash`
        leaf_hash,
        address,
        merkle_path,
    );

    PublicInput {
        root_hash: root_hash,
    }
}
```

This code will not compile, as we have several items undeclared now! Let's
define our `merkle` module and resolve the function paths:

```rust,no_run,noplaypen
mod merkle; // defined a module

struct PublicInput {
    root_hash: merkle::Sha256Digest, // use a type declaration from `merkle`
}

fn main(
    address: [bool; 10],
    balance: field,
    merkle_path: [merkle::Sha256Digest; 10] // use a type declaration from `merkle`
) -> PublicInput {
    let leaf_hash = merkle::balance_hash(balance); // call a function from `merkle`

    // call a function from `merkle`
    let root_hash = merkle::restore_root_hash(
        leaf_hash,
        address,
        merkle_path,
    );

    PublicInput {
        root_hash: root_hash,
    }
}
```

Perfect! Now all our functions and types are defined. By the way, let's have a
glance at our `merkle` module, where you can find another improvement!

```rust,no_run,noplaypen
use std::crypto::sha256; // an import

type Sha256Digest = [bool; 256];

fn balance_hash(balance: field) -> Sha256Digest {
    let bits = std::convert::to_bits(balance);
    let bits_padded = std::array::pad(bits, 256, false);
    sha256(bits_padded)
}

fn merkle_node_hash(left: Sha256Digest, right: Sha256Digest) -> Sha256Digest {
    let mut data = [false; 512];

    for i in 0..256 {
        data[i] = left[i];
        data[256 + i] = right[i];
    }

    sha256(data)
}

fn restore_root_hash(
    leaf_hash: Sha256Digest,
    address: [bool; 10],
    merkle_path: [Sha256Digest; 10],
) -> Sha256Digest
{
    let mut current = leaf_hash;

    for i in 0..10 {
        let left_and_right = if address[i] {
            (current, merkle_path[i])
        } else {
            (merkle_path[i], current)
        };

        current = merkle_node_hash(left_and_right.0, left_and_right.1);
    }

    current
}
```

You may notice a `use` statement at the first line of code. It is an import statement
which is designed to prevent using long repeated paths in our code. As you see,
now we are able to call the standard library function more conveniently:
`sha256(data)` instead of `std::crypto::sha256(data)`.

## Finalizing

Congratulations, you are an experienced Zinc developer!
Now, you may build the circuit, generate and verify a proof, like it was
explained in the [previous chapter](./02-minimal-example.md),
and move on to reading the rest of the book!
