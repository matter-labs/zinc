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

#[allow(clippy::cognitive_complexity)]
pub fn decode_instruction(bytes: &[u8]) -> Result<(Instruction, usize), DecodingError> {
    if bytes.is_empty() {
        return Err(DecodingError::UnexpectedEOF);
    }

    match bytes[0] {
        x if x == InstructionCode::NoOperation as u8 => decode_and_wrap::<NoOperation>(bytes),
        x if x == InstructionCode::PushConst as u8 => decode_and_wrap::<PushConst>(bytes),
        x if x == InstructionCode::Pop as u8 => decode_and_wrap::<Pop>(bytes),
        x if x == InstructionCode::CopyGlobal as u8 => decode_and_wrap::<CopyGlobal>(bytes),
        x if x == InstructionCode::LoadPush as u8 => decode_and_wrap::<LoadPush>(bytes),
        x if x == InstructionCode::PopStore as u8 => decode_and_wrap::<PopStore>(bytes),
        x if x == InstructionCode::LoadPushArray as u8 => decode_and_wrap::<LoadPushArray>(bytes),
        x if x == InstructionCode::PopStoreArray as u8 => decode_and_wrap::<PopStoreArray>(bytes),
        x if x == InstructionCode::LoadPushByIndex as u8 => decode_and_wrap::<LoadPushByIndex>(bytes),
        x if x == InstructionCode::PopStoreByIndex as u8 => decode_and_wrap::<PopStoreByIndex>(bytes),
        x if x == InstructionCode::Add as u8 => decode_and_wrap::<Add>(bytes),
        x if x == InstructionCode::Sub as u8 => decode_and_wrap::<Sub>(bytes),
        x if x == InstructionCode::Mul as u8 => decode_and_wrap::<Mul>(bytes),
        x if x == InstructionCode::Div as u8 => decode_and_wrap::<Div>(bytes),
        x if x == InstructionCode::Rem as u8 => decode_and_wrap::<Rem>(bytes),
        x if x == InstructionCode::Neg as u8 => decode_and_wrap::<Neg>(bytes),
        x if x == InstructionCode::Not as u8 => decode_and_wrap::<Not>(bytes),
        x if x == InstructionCode::And as u8 => decode_and_wrap::<And>(bytes),
        x if x == InstructionCode::Or as u8 => decode_and_wrap::<Or>(bytes),
        x if x == InstructionCode::Xor as u8 => decode_and_wrap::<Xor>(bytes),
        x if x == InstructionCode::Lt as u8 => decode_and_wrap::<Lt>(bytes),
        x if x == InstructionCode::Le as u8 => decode_and_wrap::<Le>(bytes),
        x if x == InstructionCode::Eq as u8 => decode_and_wrap::<Eq>(bytes),
        x if x == InstructionCode::Ne as u8 => decode_and_wrap::<Ne>(bytes),
        x if x == InstructionCode::Ge as u8 => decode_and_wrap::<Ge>(bytes),
        x if x == InstructionCode::Gt as u8 => decode_and_wrap::<Gt>(bytes),
        x if x == InstructionCode::Cast as u8 => decode_and_wrap::<Cast>(bytes),
        x if x == InstructionCode::ConditionalSelect as u8 => {
            decode_and_wrap::<ConditionalSelect>(bytes)
        }
        x if x == InstructionCode::If as u8 => decode_and_wrap::<If>(bytes),
        x if x == InstructionCode::Else as u8 => decode_and_wrap::<Else>(bytes),
        x if x == InstructionCode::EndIf as u8 => decode_and_wrap::<EndIf>(bytes),
        x if x == InstructionCode::LoopBegin as u8 => decode_and_wrap::<LoopBegin>(bytes),
        x if x == InstructionCode::LoopEnd as u8 => decode_and_wrap::<LoopEnd>(bytes),
        x if x == InstructionCode::Call as u8 => decode_and_wrap::<Call>(bytes),
        x if x == InstructionCode::Return as u8 => decode_and_wrap::<Return>(bytes),
        x if x == InstructionCode::Assert as u8 => decode_and_wrap::<Assert>(bytes),
        x if x == InstructionCode::Log as u8 => decode_and_wrap::<Dbg>(bytes),
        x if x == InstructionCode::Exit as u8 => decode_and_wrap::<Exit>(bytes),
        x if x == InstructionCode::MerkleInit as u8 => decode_and_wrap::<MerkleInit>(bytes),
        x if x == InstructionCode::MerkleGet as u8 => decode_and_wrap::<MerkleGet>(bytes),
        x if x == InstructionCode::MerkleSet as u8 => decode_and_wrap::<MerkleSet>(bytes),
        x => Err(DecodingError::UnknownInstructionCode(x)),
    }
}

fn decode_and_wrap<I: InstructionInfo>(
    bytes: &[u8],
) -> Result<(Instruction, usize), DecodingError> {
    I::decode(bytes).map(|(i, l)| (i.wrap(), l))
}
