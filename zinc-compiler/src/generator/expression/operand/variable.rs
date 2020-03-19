//!
//! The generator expression variable operand.
//!

#[derive(Debug, Clone)]
pub struct Variable {
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Variable {
    pub fn new(is_signed: bool, bitlength: usize) -> Self {
        Self {
            is_signed,
            bitlength,
        }
    }
}
