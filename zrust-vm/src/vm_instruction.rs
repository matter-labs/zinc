use crate::{Stack, RuntimeError};
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use zrust_bytecode::*;
use std::cmp;

pub trait VMInstruction<E, CS>: Instruction where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>;
}

pub fn decode_all_vm_instructions<E, CS>(bytes: &[u8])
    -> Result<Vec<Box<dyn VMInstruction<E, CS>>>, DecodingError>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    let mut instructions = Vec::new();

    let mut offset = 0;
    while offset < bytes.len() {
        match decode_vm_instruction(&bytes[offset..]) {
            Ok((instr, len)) => {
                instructions.push(instr);
                offset += len;
            },
            Err(err) => {
                let last = cmp::min(bytes.len(), offset + 10);
                log::warn!("failed to decode bytes {:?} at offset {}", &bytes[offset..last], offset);
                return Err(err);
            }
        };
    }

    Ok(instructions)
}

pub fn decode_vm_instruction<E, CS>(bytes: &[u8])
    -> Result<(Box<dyn VMInstruction<E, CS>>, usize), DecodingError>
where
    E: Engine,
    CS: ConstraintSystem<E>
{
    if bytes.len() < 1 {
        return Err(DecodingError::UnexpectedEOF);
    }

    match bytes[0] {
        x if x == InstructionCode::NoOperation as u8 =>
            NoOperation::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Push as u8 =>
            Push::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Pop as u8 =>
            Pop::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Copy as u8 =>
            Copy::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Add as u8 =>
            Add::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Sub as u8 =>
            Sub::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Mul as u8 =>
            Mul::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Div as u8 =>
            Div::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Rem as u8 =>
            Rem::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Not as u8 =>
            Not::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::And as u8 =>
            And::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Or as u8 =>
            Or::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Xor as u8 =>
            Xor::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, CS>>, usize) {(Box::new(s), len)}),

        code => Err(DecodingError::UnknownInstructionCode(code))
    }
}
