//!
//! The keyword lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
use std::str;

use failure::Fail;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // domain
    Inputs,
    Witness,
    Require,
    Debug,

    // declaration
    Let,
    Mut,

    // control
    For,
    In,
    If,
    Else,

    // type
    Bool,
    Uint { bitlength: usize },
    Int { bitlength: usize },
    Field,

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
    #[fail(display = "integer bitlength is empty")]
    IntegerBitlengthEmpty,
    #[fail(display = "integer bitlength '{}' is not numeric", _0)]
    IntegerBitlengthNotNumeric(String),
    #[fail(display = "integer bitlength {} is not multiple of {}", _0, _1)]
    IntegerBitlengthInvalidModulo(usize, usize),
    #[fail(display = "integer bitlength {} is out of range {:?}", _0, _1)]
    IntegerBitlengthOutOfRange(usize, RangeInclusive<usize>),
    #[fail(display = "unknown")]
    Unknown(String),
}

impl TryFrom<&str> for Keyword {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        const BITLENGTH_MIN: usize = 1;
        const BITLENGTH_MAX: usize = 253;
        const BITLENGTH_MODULO: usize = 8;
        const BITLENGTH_RANGE: RangeInclusive<usize> = (BITLENGTH_MIN..=BITLENGTH_MAX);

        if let Some("uint") = input.get(..4) {
            let bitlength = &input[4..];
            if bitlength.is_empty() {
                return Err(Error::IntegerBitlengthEmpty);
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::IntegerBitlengthNotNumeric(bitlength.to_owned()))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    BITLENGTH_RANGE,
                ));
            }
            if bitlength % BITLENGTH_MODULO != 0 {
                return Err(Error::IntegerBitlengthInvalidModulo(
                    bitlength,
                    BITLENGTH_MODULO,
                ));
            }
            return Ok(Self::uint(bitlength));
        }

        if let Some("int") = input.get(..3) {
            let bitlength = &input[3..];
            if bitlength.is_empty() {
                return Err(Error::IntegerBitlengthEmpty);
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::IntegerBitlengthNotNumeric(bitlength.to_owned()))?;
            if !BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    BITLENGTH_RANGE,
                ));
            }
            if bitlength % BITLENGTH_MODULO != 0 {
                return Err(Error::IntegerBitlengthInvalidModulo(
                    bitlength,
                    BITLENGTH_MODULO,
                ));
            }
            return Ok(Self::int(bitlength));
        }

        match input {
            "inputs" => Ok(Self::Inputs),
            "witness" => Ok(Self::Witness),
            "require" => Ok(Self::Require),
            "debug" => Ok(Self::Debug),

            "let" => Ok(Self::Let),
            "mut" => Ok(Self::Mut),

            "for" => Ok(Self::For),
            "in" => Ok(Self::In),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),

            "bool" => Ok(Self::Bool),
            "field" => Ok(Self::Field),

            "true" => Ok(Self::True),
            "false" => Ok(Self::False),

            "as" => Ok(Self::As),

            unknown => Err(Error::Unknown(unknown.to_owned())),
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

            Self::For => write!(f, "for"),
            Self::In => write!(f, "in"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),

            Self::Bool => write!(f, "bool"),
            Self::Uint { bitlength } => write!(f, "uint{}", bitlength),
            Self::Int { bitlength } => write!(f, "int{}", bitlength),
            Self::Field => write!(f, "field"),

            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),

            Self::As => write!(f, "as"),
        }
    }
}
