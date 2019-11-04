//!
//! ZRust bytecode instruction opcode.
//!

pub enum OperationCode {
    NoOperation = 0,
    Pop = 1,
    Push = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
    Remainder = 7,
    Negate = 8,
}
