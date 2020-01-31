# Merkle proof

In this chapter we will implement a circuit able to validate the merkle tree
root hash.

At this stage of reading the book you may be unfamiliar with some language
concepts. So, if you struggle to understand some examples, you are welcome to
read the rest of the book first, and then come back here.

Our circuit will accept the tree node path, address, and the balance
stored as its secret witness data. The public data will be tree root hash.

## Creating a new project

Let's create a new circuit called `merkle-proof`:

```bash
zargo new merkle-proof
cd merkle-proof
```

Now, you can open the project in your favorite IDE and go to `src/main.zn`,
where we are going to start writing the circuit code.

## Defining types

Let's start from defining the secret witness data arguments and the public data
return type.

```rust
fn main(
    address: [bool; 10], // the node address in the merkle tree
    balance: field, // the balance stored in the node
    merkle_path: [hash::Digest; 10] // the hash path to the node
) -> PublicInput {
    // ...
}
```
