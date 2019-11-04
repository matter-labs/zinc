//!
//! ZRust bytecode PUSH instruction.
//!

use std::fmt;

use crate::OperationCode;

pub struct Push {
    pub is_signed: bool,
    pub size: usize,
    pub data: Vec<u8>,
}

impl Push {
    pub fn new(
        is_signed: bool,
        size: usize,
        data: Vec<u8>,
    ) -> Self {
        Self { is_signed, size, data }
    }
}

impl Into<Vec<u8>> for Push {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(2 + self.data.len());
        result.push(OperationCode::Push as u8);
        result.push(if self.is_signed { 0b10000000 } else { 0 } + self.size as u8);
        result.extend(self.data);
        result
    }
}

impl fmt::Display for Push {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data = String::with_capacity(self.data.len() * (crate::BITLENGTH_BYTE + 1));
        for byte in self.data.iter() {
            data.extend(format!("{:08b} ", byte).chars());
        }
        write!(f, "{}{} {}", if self.is_signed {"S"} else {"U"}, self.size, data)
    }
}
