//!
//! The generator expression constant operand.
//!

use num_bigint::BigInt;

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Constant {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }
}
