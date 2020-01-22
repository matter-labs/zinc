use crate::instructions::utils::decode_simple_instruction;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
//#[deprecated(note = "Use If-Else-EndIf instead")]
pub struct ConditionalSelect;

impl InstructionInfo for ConditionalSelect {
    fn to_assembly(&self) -> String {
        "cs".into()
    }

    fn code() -> InstructionCode {
        InstructionCode::ConditionalSelect
    }

    fn encode(&self) -> Vec<u8> {
        vec![InstructionCode::ConditionalSelect as u8]
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        decode_simple_instruction(bytes)
    }

    fn inputs_count(&self) -> usize {
        3
    }

    fn outputs_count(&self) -> usize {
        1
    }

    fn wrap(&self) -> Instruction {
        Instruction::ConditionalSelect((*self).clone())
    }
}
