extern crate franklin_crypto;

use crate::vm::{Operator, RuntimeError};
use crate::stack::Stack;
use num_bigint::BigInt;
use ff::{PrimeField};
use bellman::pairing::Engine;
use franklin_crypto::bellman::{ConstraintSystem, Variable};
use std::io;

/// Decodes constant from bytecode and pushes it onto stack.
/// See bytecode specification for details.
pub struct Push;

const MAX_CONSTANT_LENGTH: u8 = 32;

impl<E: Engine, CS: ConstraintSystem<E>> Operator<E, CS> for Push {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<Variable>,
        bytecode: &mut dyn io::Read)
        -> Result<(), RuntimeError>
    {
        let len = Self::read_length(bytecode)?;
        let constant = Self::read_constant(len, bytecode)?;

        if let Some(fr) = E::Fr::from_str(&constant.to_string()) {
            match cs.alloc(|| "push", || Ok(fr)) {
                Ok(var) => {
                    stack.push(var);
                    Ok(())
                },
                Err(_) => Err(RuntimeError::InternalError)
            }
        } else {
            Err(RuntimeError::InternalError)
        }
    }
}

impl Push {
    fn read_length(bytecode: &mut dyn io::Read) -> Result<u8, RuntimeError> {
        let mut len_bytes: [u8; 1] = [0];

        match bytecode.read(&mut len_bytes) {
            Ok(1) => {
                let len = len_bytes[0];
                if len >= 1 && len <= MAX_CONSTANT_LENGTH {
                    Ok(len)
                } else {
                    Err(RuntimeError::InvalidArguments)
                }
            },
            Ok(_) => Err(RuntimeError::UnexpectedEndOfFile),
            Err(e) => Err(RuntimeError::IOError(e)),
        }
    }

    fn read_constant(len: u8, bytecode: &mut dyn io::Read) -> Result<BigInt, RuntimeError> {
        let mut bytes: [u8; MAX_CONSTANT_LENGTH as usize] = [0; MAX_CONSTANT_LENGTH as usize];

        match bytecode.read(&mut bytes[..(len as usize)]) {
            Ok(n) if n == len as usize => Ok(Self::parse_le_constant(&bytes[0..(len as usize)])),
            Ok(_) => Err(RuntimeError::UnexpectedEndOfFile),
            Err(e) => Err(RuntimeError::IOError(e)),
        }
    }

    fn parse_le_constant(bytes: &[u8]) -> BigInt {
        let mut constant = BigInt::from(0);

        for (i, &b) in bytes.iter().enumerate() {
            constant += (b as usize) << (8 * i);
        }

        constant
    }
}
