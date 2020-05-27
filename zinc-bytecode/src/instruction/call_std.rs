//!
//! The 'standard library function call' instruction.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::instruction::Instruction;
use crate::InstructionInfo;

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
}

/// Built-in function identifier.
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

impl InstructionInfo for CallStd {
    fn to_assembly(&self) -> String {
        format!(
            "call_builtin {:?}({}) -> {}",
            self.identifier, self.inputs_count, self.outputs_count
        )
    }

    fn wrap(self) -> Instruction {
        Instruction::CallStd(self)
    }
}
