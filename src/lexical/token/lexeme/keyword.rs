//!
//! The keyword lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;

use failure::Fail;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Keyword {
    // domain
    Inputs,
    Witness,
    Require,
    Debug,

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
    Bool,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,
    Struct,
    Enum,
    MemoryVector,
    StorageVector,

    // literal
    True,
    False,

    // operator
    As,
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
        const BITLENGTH_RANGE: RangeInclusive<usize> = (2..=253);

        if let Some(b"uint") = bytes.get(..4) {
            let bitlength = String::from_utf8_lossy(&bytes[4..]).to_string();
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Keyword::Uint { bitlength });
        }

        if let Some(b"int") = bytes.get(..3) {
            let bitlength = String::from_utf8_lossy(&bytes[3..]).to_string();
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Keyword::Int { bitlength });
        }

        match bytes {
            b"inputs" => Ok(Keyword::Inputs),
            b"witness" => Ok(Keyword::Witness),
            b"require" => Ok(Keyword::Require),
            b"debug" => Ok(Keyword::Debug),

            b"let" => Ok(Keyword::Let),
            b"mut" => Ok(Keyword::Mut),
            b"type" => Ok(Keyword::Type),

            b"for" => Ok(Keyword::For),
            b"if" => Ok(Keyword::If),
            b"else" => Ok(Keyword::Else),
            b"match" => Ok(Keyword::Match),

            b"bool" => Ok(Keyword::Bool),
            b"field" => Ok(Keyword::Field),
            b"struct" => Ok(Keyword::Struct),
            b"enum" => Ok(Keyword::Enum),
            b"memory_vector" => Ok(Keyword::MemoryVector),
            b"storage_vector" => Ok(Keyword::StorageVector),

            b"true" => Ok(Keyword::True),
            b"false" => Ok(Keyword::False),

            b"as" => Ok(Keyword::As),

            _unknown => Err(Error::Unknown),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Inputs => write!(f, "inputs"),
            Keyword::Witness => write!(f, "witness"),
            Keyword::Require => write!(f, "require"),
            Keyword::Debug => write!(f, "debug"),

            Keyword::Let => write!(f, "let"),
            Keyword::Mut => write!(f, "mut"),
            Keyword::Type => write!(f, "type"),

            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::Match => write!(f, "match"),

            Keyword::Bool => write!(f, "bool"),
            Keyword::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Keyword::Int { bitlength } => write!(f, "int{}", bitlength),
            Keyword::Field => write!(f, "field"),
            Keyword::Struct => write!(f, "struct"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::MemoryVector => write!(f, "memory_vector"),
            Keyword::StorageVector => write!(f, "storage_vector"),

            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),

            Keyword::As => write!(f, "as"),
        }
    }
}
