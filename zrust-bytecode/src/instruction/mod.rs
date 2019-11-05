//!
//! ZRust bytecode instruction.
//!

mod opcode;
mod push;

pub use self::opcode::OperationCode;
pub use self::push::Error as PushError;
pub use self::push::Push;

use std::convert::TryFrom;
use std::fmt;

use failure::Fail;

pub enum Instruction {
    NoOperation,
    Pop,
    Push(Push),
    Copy,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Negate,
    Not,
    And,
    Or,
    Xor,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "operation code missing")]
    OperationCodeMissing,
    #[fail(display = "operation code unknown: {}", _0)]
    OperationCode(u8),
    #[fail(display = "push: {}", _0)]
    Push(PushError),
}

impl Instruction {
    pub fn new_from_slice(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let opcode = match bytes.get(0).copied() {
            Some(opcode) => OperationCode::try_from(opcode).map_err(Error::OperationCode)?,
            None => return Err(Error::OperationCodeMissing),
        };

        Ok(match opcode {
            OperationCode::NoOperation => (Self::NoOperation, 1),
            OperationCode::Pop => (Self::Pop, 1),
            OperationCode::Push => {
                let (push, size) = Push::new_from_slice(&bytes[1..]).map_err(Error::Push)?;
                (Self::Push(push), 1 + size)
            }
            OperationCode::Copy => (Self::Copy, 1),
            OperationCode::Add => (Self::Add, 1),
            OperationCode::Subtract => (Self::Subtract, 1),
            OperationCode::Multiply => (Self::Multiply, 1),
            OperationCode::Divide => (Self::Divide, 1),
            OperationCode::Remainder => (Self::Remainder, 1),
            OperationCode::Negate => (Self::Negate, 1),
            OperationCode::Not => (Self::Not, 1),
            OperationCode::And => (Self::And, 1),
            OperationCode::Or => (Self::Or, 1),
            OperationCode::Xor => (Self::Xor, 1),
        })
    }

    pub fn opcode(&self) -> OperationCode {
        match self {
            Self::NoOperation => OperationCode::NoOperation,
            Self::Pop => OperationCode::Pop,
            Self::Push { .. } => OperationCode::Push,
            Self::Copy => OperationCode::Copy,
            Self::Add => OperationCode::Add,
            Self::Subtract => OperationCode::Subtract,
            Self::Multiply => OperationCode::Multiply,
            Self::Divide => OperationCode::Divide,
            Self::Remainder => OperationCode::Remainder,
            Self::Negate => OperationCode::Negate,
            Self::Not => OperationCode::Not,
            Self::And => OperationCode::And,
            Self::Or => OperationCode::Or,
            Self::Xor => OperationCode::Xor,
        }
    }
}

impl Into<Vec<u8>> for Instruction {
    fn into(self) -> Vec<u8> {
        match self {
            Self::NoOperation => vec![OperationCode::NoOperation as u8],
            Self::Pop => vec![OperationCode::Pop as u8],
            Self::Push(instruction) => instruction.into(),
            Self::Copy => vec![OperationCode::Copy as u8],
            Self::Add => vec![OperationCode::Add as u8],
            Self::Subtract => vec![OperationCode::Subtract as u8],
            Self::Multiply => vec![OperationCode::Multiply as u8],
            Self::Divide => vec![OperationCode::Divide as u8],
            Self::Remainder => vec![OperationCode::Remainder as u8],
            Self::Negate => vec![OperationCode::Negate as u8],
            Self::Not => vec![OperationCode::Not as u8],
            Self::And => vec![OperationCode::And as u8],
            Self::Or => vec![OperationCode::Or as u8],
            Self::Xor => vec![OperationCode::Xor as u8],
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoOperation => write!(f, "noop"),
            Self::Pop => write!(f, "pop"),
            Self::Push(inner) => write!(f, "push {}", inner),
            Self::Copy => write!(f, "copy"),
            Self::Add => write!(f, "add"),
            Self::Subtract => write!(f, "subtract"),
            Self::Multiply => write!(f, "multiply"),
            Self::Divide => write!(f, "divide"),
            Self::Remainder => write!(f, "remainder"),
            Self::Negate => write!(f, "negate"),
            Self::Not => write!(f, "not"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
        }
    }
}
