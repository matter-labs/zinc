//!
//! The 'standard library function call' instruction.
//!

use std::fmt;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;

///
/// Built-in function identifier.
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BuiltinIdentifier {
    CryptoSha256,
    CryptoPedersen,
    ToBits,
    UnsignedFromBits,
    SignedFromBits,
    FieldFromBits,
    ArrayReverse,
    ArrayTruncate,
    ArrayPad,
    CryptoSchnorrSignatureVerify,
    FieldInverse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallStd {
    pub identifier: BuiltinIdentifier,
    pub inputs_count: usize,
    pub outputs_count: usize,
}

impl CallStd {
    pub fn new(identifier: BuiltinIdentifier, inputs_count: usize, outputs_count: usize) -> Self {
        Self {
            identifier,
            inputs_count,
            outputs_count,
        }
    }

    pub fn is_debug(&self) -> bool {
        false
    }

    pub fn wrap(self) -> Instruction {
        Instruction::CallStd(self)
    }
}

impl fmt::Display for CallStd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "call_std {:?}({}) -> {}",
            self.identifier, self.inputs_count, self.outputs_count
        )
    }
}
