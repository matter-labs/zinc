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

impl Keyword {
    pub fn uint(bitlength: usize) -> Self {
        Self::Uint { bitlength }
    }

    pub fn int(bitlength: usize) -> Self {
        Self::Int { bitlength }
    }
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
        const BITLENGTH_MIN: usize = 2;
        const BITLENGTH_MAX: usize = 253;
        const BITLENGTH_RANGE: RangeInclusive<usize> = (BITLENGTH_MIN..=BITLENGTH_MAX);

        if let Some(b"uint") = bytes.get(..4) {
            let bitlength = String::from_utf8_lossy(&bytes[4..]).to_string();
            if bitlength.is_empty() {
                return Ok(Self::uint(BITLENGTH_MAX));
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Self::uint(bitlength));
        }

        if let Some(b"int") = bytes.get(..3) {
            let bitlength = String::from_utf8_lossy(&bytes[3..]).to_string();
            if bitlength.is_empty() {
                return Ok(Self::int(BITLENGTH_MAX));
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::BitlengthNotNumeric(bitlength))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::BitlengthOutOfRange(bitlength, BITLENGTH_RANGE));
            }
            return Ok(Self::int(bitlength));
        }

        match bytes {
            b"inputs" => Ok(Self::Inputs),
            b"witness" => Ok(Self::Witness),
            b"require" => Ok(Self::Require),
            b"debug" => Ok(Self::Debug),

            b"let" => Ok(Self::Let),
            b"mut" => Ok(Self::Mut),
            b"type" => Ok(Self::Type),

            b"for" => Ok(Self::For),
            b"if" => Ok(Self::If),
            b"else" => Ok(Self::Else),
            b"match" => Ok(Self::Match),

            b"bool" => Ok(Self::Bool),
            b"field" => Ok(Self::Field),
            b"struct" => Ok(Self::Struct),
            b"enum" => Ok(Self::Enum),
            b"memory_vector" => Ok(Self::MemoryVector),
            b"storage_vector" => Ok(Self::StorageVector),

            b"true" => Ok(Self::True),
            b"false" => Ok(Self::False),

            b"as" => Ok(Self::As),

            _unknown => Err(Error::Unknown),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inputs => write!(f, "inputs"),
            Self::Witness => write!(f, "witness"),
            Self::Require => write!(f, "require"),
            Self::Debug => write!(f, "debug"),

            Self::Let => write!(f, "let"),
            Self::Mut => write!(f, "mut"),
            Self::Type => write!(f, "type"),

            Self::For => write!(f, "for"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Match => write!(f, "match"),

            Self::Bool => write!(f, "bool"),
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),
            Self::Struct => write!(f, "struct"),
            Self::Enum => write!(f, "enum"),
            Self::MemoryVector => write!(f, "memory_vector"),
            Self::StorageVector => write!(f, "storage_vector"),

            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),

            Self::As => write!(f, "as"),
        }
    }
}
