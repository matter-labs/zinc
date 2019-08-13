//!
//! The keyword lexeme.
//!

use std::convert::TryFrom;
use std::ops::RangeInclusive;

use failure::Fail;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Keyword {
    // domain
    Inputs,
    Witness,
    Require,

    // declaration
    Let,
    Mut,
    Type,

    // control
    For,
    If,
    Else,
    Match,

    // type
    Uint(usize),
    Int(usize),
    Field,
    Bool,
    Struct,
    Enum,
    MemoryVector,
    StorageVector,

    // boolean
    True,
    False,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "integer bitlength '{}' is not numeric", _0)]
    BitlengthNotNumeric(String),
    #[fail(display = "integer bitlength {} is out of range {:?}", _0, _1)]
    BitlengthOutOfRange(usize, RangeInclusive<usize>),
    #[fail(display = "unknown")]
    Unknown,
}

impl TryFrom<&[u8]> for Keyword {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        const BITLENGTH_RANGE: RangeInclusive<usize> = (1..=126);

        if let Some(b"uint") = bytes.get(..4) {
            let bitlength = String::from_utf8_lossy(&bytes[4..]).to_string();
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Keyword::Uint(bitlength));
        }

        if let Some(b"int") = bytes.get(..3) {
            let bitlength = String::from_utf8_lossy(&bytes[3..]).to_string();
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Keyword::Int(bitlength));
        }

        match bytes {
            b"inputs" => Ok(Keyword::Inputs),
            b"witness" => Ok(Keyword::Witness),
            b"require" => Ok(Keyword::Require),

            b"let" => Ok(Keyword::Let),
            b"mut" => Ok(Keyword::Mut),
            b"type" => Ok(Keyword::Type),

            b"for" => Ok(Keyword::For),
            b"if" => Ok(Keyword::If),
            b"else" => Ok(Keyword::Else),
            b"match" => Ok(Keyword::Match),

            b"field" => Ok(Keyword::Field),
            b"bool" => Ok(Keyword::Bool),
            b"struct" => Ok(Keyword::Struct),
            b"enum" => Ok(Keyword::Enum),
            b"memory_vector" => Ok(Keyword::MemoryVector),
            b"storage_vector" => Ok(Keyword::StorageVector),

            b"true" => Ok(Keyword::True),
            b"false" => Ok(Keyword::False),

            _unknown => Err(Error::Unknown),
        }
    }
}
