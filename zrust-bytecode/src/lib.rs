//!
//! ZRust bytecode library.
//!

mod instruction;

pub use self::instruction::Instruction;
pub use self::instruction::Push;
pub use self::instruction::OperationCode;

pub const BITLENGTH_BYTE: usize = 8;
