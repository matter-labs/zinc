use crate::{RuntimeError, VirtualMachine, Element, ElementOperator};
use zrust_bytecode::*;
use std::cmp;

pub trait VMInstruction<E, O>: Instruction
where
    E: Element,
    O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError>;
}

pub fn decode_all_vm_instructions<E, O>(bytes: &[u8])
    -> Result<Vec<Box<dyn VMInstruction<E, O>>>, DecodingError>
where
    E: Element,
    O: ElementOperator<E>
{
    log::info!("Started parsing...");

    let mut instructions = Vec::new();

    let mut offset = 0;
    while offset < bytes.len() {
        match decode_vm_instruction(&bytes[offset..]) {
            Ok((instr, len)) => {
                log::info!("{} \t{:2x?}", instr.to_assembly(), &bytes[offset..(offset+len)]);
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

    log::info!("Done parsing.");

    Ok(instructions)
}

pub fn decode_vm_instruction<E, O>(bytes: &[u8])
    -> Result<(Box<dyn VMInstruction<E, O>>, usize), DecodingError>
where
    E: Element,
    O: ElementOperator<E>
{
    if bytes.len() < 1 {
        return Err(DecodingError::UnexpectedEOF);
    }

    match bytes[0] {
        x if x == InstructionCode::NoOperation as u8 =>
            NoOperation::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Push as u8 =>
            Push::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Pop as u8 =>
            Pop::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Copy as u8 =>
            Copy::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Add as u8 =>
            Add::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Sub as u8 =>
            Sub::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Mul as u8 =>
            Mul::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Div as u8 =>
            Div::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Rem as u8 =>
            Rem::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Not as u8 =>
            Not::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::And as u8 =>
            And::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Or as u8 =>
            Or::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Xor as u8 =>
            Xor::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Lt as u8 =>
            Lt::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Le as u8 =>
            Le::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Eq as u8 =>
            Eq::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Ne as u8 =>
            Ne::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Ge as u8 =>
            Ge::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::Gt as u8 =>
            Gt::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::ConditionalSelect as u8 =>
            ConditionalSelect::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::LoopBegin as u8 =>
            LoopBegin::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        x if x == InstructionCode::LoopEnd as u8 =>
            LoopEnd::decode(bytes).map(|(s, len)| -> (Box<dyn VMInstruction<E, O>>, usize) {(Box::new(s), len)}),

        code => Err(DecodingError::UnknownInstructionCode(code))
    }
}
