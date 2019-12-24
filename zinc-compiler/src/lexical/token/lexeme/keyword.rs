//!
//! The lexical token keyword lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
use std::str;

use failure::Fail;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // declarations
    Let,
    Mut,
    Const,
    Static,
    Type,
    Struct,
    Enum,
    Fn,
    Mod,
    Use,
    Impl,

    // controls
    For,
    In,
    While,
    If,
    Else,
    Match,

    // types
    Bool,
    IntegerUnsigned { bitlength: usize },
    IntegerSigned { bitlength: usize },
    Field,

    // literals
    True,
    False,

    // operators
    As,
}

impl Keyword {
    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
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
        match input {
            "let" => return Ok(Self::Let),
            "mut" => return Ok(Self::Mut),
            "const" => return Ok(Self::Const),
            "static" => return Ok(Self::Static),
            "type" => return Ok(Self::Type),
            "struct" => return Ok(Self::Struct),
            "enum" => return Ok(Self::Enum),
            "fn" => return Ok(Self::Fn),
            "mod" => return Ok(Self::Mod),
            "use" => return Ok(Self::Use),
            "impl" => return Ok(Self::Impl),

            "for" => return Ok(Self::For),
            "in" => return Ok(Self::In),
            "while" => return Ok(Self::While),
            "if" => return Ok(Self::If),
            "else" => return Ok(Self::Else),
            "match" => return Ok(Self::Match),

            "bool" => return Ok(Self::Bool),
            "field" => return Ok(Self::Field),

            "true" => return Ok(Self::True),
            "false" => return Ok(Self::False),

            "as" => return Ok(Self::As),

            _ => {}
        }

        const BITLENGTH_MIN: usize = 1;
        const BITLENGTH_MAX: usize = 253;
        const BITLENGTH_MODULO: usize = 8;
        const BITLENGTH_RANGE: RangeInclusive<usize> = (BITLENGTH_MIN..=BITLENGTH_MAX);

        // The following code checks if the number after the 'u' or 'i' symbol represents a valid
        // amount of bits for an integer value. If the amount is not a valid bitlength, the word is
        // considered as an ordinar identifier.

        if let Some("u") = input.get(..1) {
            let bitlength = &input[1..];
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
            return Ok(Self::new_integer_unsigned(bitlength));
        }

        if let Some("i") = input.get(..1) {
            let bitlength = &input[1..];
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
            return Ok(Self::new_integer_signed(bitlength));
        }

        Err(Error::Unknown(input.to_owned()))
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::Mut => write!(f, "mut"),
            Self::Const => write!(f, "const"),
            Self::Static => write!(f, "static"),
            Self::Type => write!(f, "type"),
            Self::Struct => write!(f, "struct"),
            Self::Enum => write!(f, "enum"),
            Self::Fn => write!(f, "fn"),
            Self::Mod => write!(f, "mod"),
            Self::Use => write!(f, "use"),
            Self::Impl => write!(f, "impl"),

            Self::For => write!(f, "for"),
            Self::In => write!(f, "in"),
            Self::While => write!(f, "while"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Match => write!(f, "match"),

            Self::Bool => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),

            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),

            Self::As => write!(f, "as"),
        }
    }
}
