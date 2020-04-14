# Schnorr signature tool

The `schnorr` signature tool can perform the following actions:

- generate a private key
- sign a message with a private key
- extract the public key from a private key

### Generating a private key

```bash
schnorr gen-key > 'private_key.txt'
```

### Signing a message with a private key

From directory.file:

```bash
schnorr sign --key 'private_key.txt' --message 'message.txt'
```

From `stdin`:

```bash
schnorr sign --key 'private_key.txt' --message -
```

The JSON output can be used as witness data if you want to pass the signature to a circuit.

### Extracting the public key

```bash
schnorr pub-key < 'private_key.txt' > 'public_key.txt'
```
