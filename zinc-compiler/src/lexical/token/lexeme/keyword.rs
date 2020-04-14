//!
//! The lexical token keyword lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // declarations
    Let,
    Mut,
    Const,
    Type,
    Struct,
    Enum,
    Fn,
    Mod,
    Use,
    Impl,
    Contract,

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

    // aliases
    SelfUppercase,
    SelfLowercase,

    // reserved
    Pub,
    Ref,
    Extern,
    Return,
    Loop,
    Break,
    Continue,
}

impl Keyword {
    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }
}

#[derive(Debug)]
pub enum Error {
    IntegerBitlengthEmpty,
    IntegerBitlengthNotNumeric(String),
    IntegerBitlengthNotMultipleOfEight(usize, usize),
    IntegerBitlengthOutOfRange(usize, RangeInclusive<usize>),
    Unknown(String),
}

impl TryFrom<&str> for Keyword {
    type Error = Error;

    ///
    /// The converter checks if the number after the 'u' or 'i' symbol represents a valid
    /// amount of bits for an integer value. If the amount is not a valid bitlength, the word is
    /// treated as an ordinar identifier.
    ///
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "let" => return Ok(Self::Let),
            "mut" => return Ok(Self::Mut),
            "const" => return Ok(Self::Const),
            "type" => return Ok(Self::Type),
            "struct" => return Ok(Self::Struct),
            "enum" => return Ok(Self::Enum),
            "fn" => return Ok(Self::Fn),
            "mod" => return Ok(Self::Mod),
            "use" => return Ok(Self::Use),
            "impl" => return Ok(Self::Impl),
            "contract" => return Ok(Self::Contract),

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

            "Self" => return Ok(Self::SelfUppercase),
            "self" => return Ok(Self::SelfLowercase),

            "pub" => return Ok(Self::Pub),
            "ref" => return Ok(Self::Ref),
            "extern" => return Ok(Self::Extern),
            "return" => return Ok(Self::Return),
            "loop" => return Ok(Self::Loop),
            "break" => return Ok(Self::Break),
            "continue" => return Ok(Self::Continue),

            _ => {}
        }

        const INTEGER_BITLENGTH_RANGE: RangeInclusive<usize> =
            (crate::BITLENGTH_BYTE..=crate::BITLENGTH_MAX_INT);

        if let Some("u") = input.get(..1) {
            let bitlength = &input[1..];
            if bitlength.is_empty() {
                return Err(Error::IntegerBitlengthEmpty);
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::IntegerBitlengthNotNumeric(bitlength.to_owned()))?;
            if !INTEGER_BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    INTEGER_BITLENGTH_RANGE,
                ));
            }
            if bitlength % crate::BITLENGTH_BYTE != 0 {
                return Err(Error::IntegerBitlengthNotMultipleOfEight(
                    bitlength,
                    crate::BITLENGTH_BYTE,
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
            if !INTEGER_BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    INTEGER_BITLENGTH_RANGE,
                ));
            }
            if bitlength % crate::BITLENGTH_BYTE != 0 {
                return Err(Error::IntegerBitlengthNotMultipleOfEight(
                    bitlength,
                    crate::BITLENGTH_BYTE,
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
            Self::Type => write!(f, "type"),
            Self::Struct => write!(f, "struct"),
            Self::Enum => write!(f, "enum"),
            Self::Fn => write!(f, "fn"),
            Self::Mod => write!(f, "mod"),
            Self::Use => write!(f, "use"),
            Self::Impl => write!(f, "impl"),
            Self::Contract => write!(f, "contract"),

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

            Self::SelfUppercase => write!(f, "Self"),
            Self::SelfLowercase => write!(f, "self"),

            Self::Pub => write!(f, "pub"),
            Self::Ref => write!(f, "ref"),
            Self::Extern => write!(f, "extern"),
            Self::Return => write!(f, "return"),
            Self::Loop => write!(f, "loop"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
        }
    }
}
