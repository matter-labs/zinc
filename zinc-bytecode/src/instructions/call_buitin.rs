use crate::builtins::BuiltinIdentifier;
use crate::instructions::utils;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use num_traits::cast::FromPrimitive;

#[derive(Debug, PartialEq, Clone)]
pub struct CallBuiltin {
    pub identifier: BuiltinIdentifier,
    pub inputs_count: usize,
    pub outputs_count: usize,
}

impl CallBuiltin {
    pub fn new(identifier: BuiltinIdentifier, inputs_count: usize, outputs_count: usize) -> Self {
        Self {
            identifier,
            inputs_count,
            outputs_count,
        }
    }
}

impl InstructionInfo for CallBuiltin {
    fn to_assembly(&self) -> String {
        format!(
            "call_builtin {}({}) -> {}",
            self.identifier, self.inputs_count, self.outputs_count
        )
    }

    fn code() -> InstructionCode {
        InstructionCode::CallBuiltin
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(
            Self::code(),
            &[
                self.identifier as usize,
                self.inputs_count,
                self.outputs_count,
            ],
        )
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 3)?;
        let identifier =
            BuiltinIdentifier::from_usize(args[0]).ok_or(DecodingError::ConstantTooLong)?;
        Ok((Self::new(identifier, args[1], args[2]), len))
    }

    fn inputs_count(&self) -> usize {
        self.inputs_count
    }

    fn outputs_count(&self) -> usize {
        self.outputs_count
    }

    fn wrap(&self) -> Instruction {
        Instruction::CallBuiltin((*self).clone())
    }
}
