//!
//! ZRust bytecode PUSH instruction.
//!

use std::fmt;

use failure::Fail;

use crate::OperationCode;

#[derive(Debug)]
pub struct Push {
    pub is_signed: bool,
    pub size: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "sign and length byte missing")]
    SignAndLengthMissing,
    #[fail(display = "data length must be 1 <= 32, but got {}", _0)]
    InvalidDataLength(usize),
    #[fail(display = "not enough bytes: expected {}, but got {}", _0, _1)]
    NotEnoughBytes(usize, usize),
}

impl Push {
    pub const BITMASK_UNSIGNED: u8 = 0b00000000;
    pub const BITMASK_SIGNED: u8 = 0b10000000;
    pub const BITMASK_LENGTH: u8 = 0b01111111;

    pub fn new(is_signed: bool, size: usize, data: Vec<u8>) -> Self {
        Self {
            is_signed,
            size,
            data,
        }
    }

    pub fn new_from_slice(bytes: &[u8]) -> Result<(Self, usize), Error> {
        let sign_and_length = match bytes.get(0).copied() {
            Some(sign_and_length) => sign_and_length,
            None => return Err(Error::SignAndLengthMissing),
        };

        let is_signed = sign_and_length & Self::BITMASK_SIGNED == Self::BITMASK_SIGNED;

        let data_length = (sign_and_length & Self::BITMASK_LENGTH) as usize;
        if data_length < 1 || 32 < data_length {
            return Err(Error::InvalidDataLength(data_length));
        }

        let data = match bytes[1..].len() {
            length if length >= data_length => bytes[1..=data_length].to_vec(),
            length => return Err(Error::NotEnoughBytes(data_length, length)),
        };

        let size = 1 + data.len();
        let result = Self {
            is_signed,
            size: data_length,
            data,
        };

        Ok((result, size))
    }
}

impl Into<Vec<u8>> for Push {
    fn into(self) -> Vec<u8> {
        let mut result = Vec::with_capacity(2 + self.data.len());
        result.push(OperationCode::Push as u8);
        result.push(
            if self.is_signed {
                Self::BITMASK_SIGNED
            } else {
                Self::BITMASK_UNSIGNED
            } + self.size as u8,
        );
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
        write!(
            f,
            "{}{} {}",
            if self.is_signed { "S" } else { "U" },
            self.size,
            data
        )
    }
}
