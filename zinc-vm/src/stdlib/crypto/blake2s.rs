use bellman::ConstraintSystem;
use franklin_crypto::circuit::blake2s::blake2s;

use crate::{Engine, MalformedBytecode, Result};
use crate::core::EvaluationStack;
use crate::gadgets::Scalar;
use crate::stdlib::NativeFunction;

const BYTE_LENGTH: usize = 8;

pub struct Blake2s {
    message_length: usize,
}

impl Blake2s {
    pub fn new(message_length: usize) -> Result<Self> {
        if message_length % 8 == 0 {
            Ok(Self { message_length })
        } else {
            Err(MalformedBytecode::InvalidArguments(format!(
                "message length for blake2s must be a multiple of 8, got {}",
                message_length
            ))
            .into())
        }
    }
}

// Implementation of Blake2s gadget for Zinc.
// It uses blake2s implementation of the franklin_crypto library.

// IMPORTANT NOTE ABOUT THE GADGET:
// In its original format, the hash digest of the franklin_crypto library does
// not match with the original blake2 specification and with the BouncyCastle library.
// Both the original spec and the BouncyCastle requires a little-endian representation
// of **bytes** within the hash computation. And the same is for the franklin_crypto.
// However, on top of that, franklin_crypto also requires little-endian ordering of
// **bits within each byte** due to the UInt32 object type used in the implementation.
// UInt32 is a representation of 32 Boolean objects as an unsigned integer, where the
// least significant bit is located in the first place.

// To overcome the mismatch between the franklin_crypto and the original spec, we added
// a function in our gadget, reverse_byte_bits(), which reverses the bit order within
// every byte before and after hashing operation.
impl<E: Engine> NativeFunction<E> for Blake2s {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result {
        let mut bits = Vec::new();
        for i in 0..self.message_length {
            let bit = stack
                .pop()?
                .value()?
                .to_boolean(cs.namespace(|| format!("bit {}", i)))?;

            bits.push(bit);
        }
        bits.reverse();

        // This function reverses the bit order within each byte of the parameter: a list of bits
        let reverse_byte_bits =
            |input: &mut [_]| input.chunks_mut(BYTE_LENGTH).for_each(|p| p.reverse());

        //reverse preimage for compatibility with the original spec
        reverse_byte_bits(&mut bits);

        let mut digest_bits = blake2s(cs.namespace(|| "blake2s"), &bits, b"12345678")?;

        //reverse digest for compatibility with the original spec
        reverse_byte_bits(&mut digest_bits);

        assert_eq!(digest_bits.len(), 256);

        for bit in digest_bits {
            let scalar = Scalar::from_boolean(cs.namespace(|| "from_boolean"), bit)?;
            stack.push(scalar.into())?;
        }

        Ok(())
    }
}
