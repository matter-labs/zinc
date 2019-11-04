//!
//! ZRust bytecode instruction.
//!

mod push;
mod opcode;

pub use self::opcode::OperationCode;
pub use self::push::Push;

use std::fmt;

pub enum Instruction {
    NoOperation,
    Pop,
    Push(Push),
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Negate,
}

impl Instruction {
    pub fn opcode(&self) -> OperationCode {
        match self {
            Self::NoOperation => OperationCode::NoOperation,
            Self::Pop => OperationCode::Pop,
            Self::Push { .. } => OperationCode::Push,
            Self::Add => OperationCode::Add,
            Self::Subtract => OperationCode::Subtract,
            Self::Multiply => OperationCode::Multiply,
            Self::Divide => OperationCode::Divide,
            Self::Remainder => OperationCode::Remainder,
            Self::Negate => OperationCode::Negate,
        }
    }
}

impl Into<Vec<u8>> for Instruction {
    fn into(self) -> Vec<u8> {
        match self {
            Self::NoOperation => vec![OperationCode::NoOperation as u8],
            Self::Pop => vec![OperationCode::Pop as u8],
            Self::Push(instruction) => instruction.into(),
            Self::Add => vec![OperationCode::Add as u8],
            Self::Subtract => vec![OperationCode::Subtract as u8],
            Self::Multiply => vec![OperationCode::Multiply as u8],
            Self::Divide => vec![OperationCode::Divide as u8],
            Self::Remainder => vec![OperationCode::Remainder as u8],
            Self::Negate => vec![OperationCode::Negate as u8],
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoOperation => write!(f, "noop"),
            Self::Pop => write!(f, "pop"),
            Self::Push(inner) => write!(f, "push {}", inner),
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "subtract"),
            Self::Multiply => write!(f, "multiply"),
            Self::Divide => write!(f, "divide"),
            Self::Remainder => write!(f, "remainder"),
            Self::Negate => write!(f, "negate"),
        }
    }
}
