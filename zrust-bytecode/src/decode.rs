use crate::instructions::*;
use crate::{DecodingError, Instruction, InstructionCode, InstructionInfo};
use std::cmp;

pub fn decode_all_instructions(bytes: &[u8]) -> Result<Vec<Instruction>, DecodingError> {
    let mut instructions = Vec::new();

    let mut offset = 0;
    while offset < bytes.len() {
        match decode_instruction(&bytes[offset..]) {
            Ok((instr, len)) => {
                instructions.push(instr);
                offset += len;
            }
            Err(err) => {
                let last = cmp::min(bytes.len(), offset + 10);
                log::warn!(
                    "Failed to decode bytes {:?} at offset {}",
                    &bytes[offset..last],
                    offset
                );
                return Err(err);
            }
        };
    }

    Ok(instructions)
}

pub fn decode_instruction(bytes: &[u8]) -> Result<(Instruction, usize), DecodingError> {
    if bytes.len() < 1 {
        return Err(DecodingError::UnexpectedEOF);
    }

    match bytes[0] {
        x if x == InstructionCode::NoOperation as u8 => {
            NoOperation::decode(bytes).map(|(i, l)| (Instruction::NoOperation(i), l))
        }
        x if x == InstructionCode::Push as u8 => {
            Push::decode(bytes).map(|(i, l)| (Instruction::Push(i), l))
        }
        x if x == InstructionCode::Pop as u8 => {
            Pop::decode(bytes).map(|(i, l)| (Instruction::Pop(i), l))
        }
        x if x == InstructionCode::Copy as u8 => {
            Copy::decode(bytes).map(|(i, l)| (Instruction::Copy(i), l))
        }
        x if x == InstructionCode::Add as u8 => {
            Add::decode(bytes).map(|(i, l)| (Instruction::Add(i), l))
        }
        x if x == InstructionCode::Sub as u8 => {
            Sub::decode(bytes).map(|(i, l)| (Instruction::Sub(i), l))
        }
        x if x == InstructionCode::Mul as u8 => {
            Mul::decode(bytes).map(|(i, l)| (Instruction::Mul(i), l))
        }
        x if x == InstructionCode::Div as u8 => {
            Div::decode(bytes).map(|(i, l)| (Instruction::Div(i), l))
        }
        x if x == InstructionCode::Rem as u8 => {
            Rem::decode(bytes).map(|(i, l)| (Instruction::Rem(i), l))
        }
        x if x == InstructionCode::Neg as u8 => {
            Neg::decode(bytes).map(|(i, l)| (Instruction::Neg(i), l))
        }
        x if x == InstructionCode::Not as u8 => {
            Not::decode(bytes).map(|(i, l)| (Instruction::Not(i), l))
        }
        x if x == InstructionCode::And as u8 => {
            And::decode(bytes).map(|(i, l)| (Instruction::And(i), l))
        }
        x if x == InstructionCode::Or as u8 => {
            Or::decode(bytes).map(|(i, l)| (Instruction::Or(i), l))
        }
        x if x == InstructionCode::Xor as u8 => {
            Xor::decode(bytes).map(|(i, l)| (Instruction::Xor(i), l))
        }
        x if x == InstructionCode::Lt as u8 => {
            Lt::decode(bytes).map(|(i, l)| (Instruction::Lt(i), l))
        }
        x if x == InstructionCode::Le as u8 => {
            Le::decode(bytes).map(|(i, l)| (Instruction::Le(i), l))
        }
        x if x == InstructionCode::Eq as u8 => {
            Eq::decode(bytes).map(|(i, l)| (Instruction::Eq(i), l))
        }
        x if x == InstructionCode::Ne as u8 => {
            Ne::decode(bytes).map(|(i, l)| (Instruction::Ne(i), l))
        }
        x if x == InstructionCode::Ge as u8 => {
            Ge::decode(bytes).map(|(i, l)| (Instruction::Ge(i), l))
        }
        x if x == InstructionCode::Gt as u8 => {
            Gt::decode(bytes).map(|(i, l)| (Instruction::Gt(i), l))
        }
        x if x == InstructionCode::Cast as u8 => {
            Cast::decode(bytes).map(|(i, l)| (Instruction::Cast(i), l))
        }
        x if x == InstructionCode::ConditionalSelect as u8 => {
            ConditionalSelect::decode(bytes).map(|(i, l)| (Instruction::ConditionalSelect(i), l))
        }
        x if x == InstructionCode::LoopBegin as u8 => {
            LoopBegin::decode(bytes).map(|(i, l)| (Instruction::LoopBegin(i), l))
        }
        x if x == InstructionCode::LoopEnd as u8 => {
            LoopEnd::decode(bytes).map(|(i, l)| (Instruction::LoopEnd(i), l))
        }
        x if x == InstructionCode::Call as u8 => {
            Call::decode(bytes).map(|(i, l)| (Instruction::Call(i), l))
        }
        x if x == InstructionCode::Return as u8 => {
            Return::decode(bytes).map(|(i, l)| (Instruction::Return(i), l))
        }
        x if x == InstructionCode::Assert as u8 => {
            Assert::decode(bytes).map(|(i, l)| (Instruction::Assert(i), l))
        }
        x if x == InstructionCode::PushCondition as u8 => {
            PushCondition::decode(bytes).map(|(i, l)| (Instruction::PushCondition(i), l))
        }
        x if x == InstructionCode::PopCondition as u8 => {
            PopCondition::decode(bytes).map(|(i, l)| (Instruction::PopCondition(i), l))
        }
        x if x == InstructionCode::Exit as u8 => {
            Exit::decode(bytes).map(|(i, l)| (Instruction::Exit(i), l))
        }
        x => Err(DecodingError::UnknownInstructionCode(x)),
    }
}
