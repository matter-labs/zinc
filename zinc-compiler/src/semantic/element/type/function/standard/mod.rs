//!
//! The semantic analyzer standard library function type element.
//!

pub mod array_pad;
pub mod array_reverse;
pub mod array_truncate;
pub mod error;
pub mod from_bits_field;
pub mod from_bits_signed;
pub mod from_bits_unsigned;
pub mod pedersen;
pub mod sha256;
pub mod to_bits;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use self::array_pad::Function as ArrayPadFunction;
use self::array_reverse::Function as ArrayReverseFunction;
use self::array_truncate::Function as ArrayTruncateFunction;
use self::from_bits_field::Function as FromBitsFieldFunction;
use self::from_bits_signed::Function as FromBitsSignedFunction;
use self::from_bits_unsigned::Function as FromBitsUnsignedFunction;
use self::pedersen::Function as PedersenFunction;
use self::sha256::Function as Sha256Function;
use self::to_bits::Function as ToBitsFunction;

#[derive(Debug, Clone)]
pub enum Function {
    Sha256(Sha256Function),
    Pedersen(PedersenFunction),
    ToBits(ToBitsFunction),
    FromBitsUnsigned(FromBitsUnsignedFunction),
    FromBitsSigned(FromBitsSignedFunction),
    FromBitsField(FromBitsFieldFunction),
    ArrayReverse(ArrayReverseFunction),
    ArrayTruncate(ArrayTruncateFunction),
    ArrayPad(ArrayPadFunction),
}

impl Function {
    pub fn new(identifier: BuiltinIdentifier) -> Self {
        match identifier {
            BuiltinIdentifier::CryptoSha256 => Self::Sha256(Sha256Function::new()),
            BuiltinIdentifier::CryptoPedersen => Self::Pedersen(PedersenFunction::new()),
            BuiltinIdentifier::ToBits => Self::ToBits(ToBitsFunction::new()),
            BuiltinIdentifier::UnsignedFromBits => {
                Self::FromBitsUnsigned(FromBitsUnsignedFunction::new())
            }
            BuiltinIdentifier::SignedFromBits => {
                Self::FromBitsSigned(FromBitsSignedFunction::new())
            }
            BuiltinIdentifier::FieldFromBits => Self::FromBitsField(FromBitsFieldFunction::new()),
            BuiltinIdentifier::ArrayReverse => Self::ArrayReverse(ArrayReverseFunction::new()),
            BuiltinIdentifier::ArrayTruncate => Self::ArrayTruncate(ArrayTruncateFunction::new()),
            BuiltinIdentifier::ArrayPad => Self::ArrayPad(ArrayPadFunction::new()),
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Sha256(inner) => inner.identifier(),
            Self::Pedersen(inner) => inner.identifier(),
            Self::ToBits(inner) => inner.identifier(),
            Self::FromBitsUnsigned(inner) => inner.identifier(),
            Self::FromBitsSigned(inner) => inner.identifier(),
            Self::FromBitsField(inner) => inner.identifier(),
            Self::ArrayReverse(inner) => inner.identifier(),
            Self::ArrayTruncate(inner) => inner.identifier(),
            Self::ArrayPad(inner) => inner.identifier(),
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::Sha256(inner) => inner.builtin_identifier(),
            Self::Pedersen(inner) => inner.builtin_identifier(),
            Self::ToBits(inner) => inner.builtin_identifier(),
            Self::FromBitsUnsigned(inner) => inner.builtin_identifier(),
            Self::FromBitsSigned(inner) => inner.builtin_identifier(),
            Self::FromBitsField(inner) => inner.builtin_identifier(),
            Self::ArrayReverse(inner) => inner.builtin_identifier(),
            Self::ArrayTruncate(inner) => inner.builtin_identifier(),
            Self::ArrayPad(inner) => inner.builtin_identifier(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256(inner) => write!(f, "{}", inner),
            Self::Pedersen(inner) => write!(f, "{}", inner),
            Self::ToBits(inner) => write!(f, "{}", inner),
            Self::FromBitsUnsigned(inner) => write!(f, "{}", inner),
            Self::FromBitsSigned(inner) => write!(f, "{}", inner),
            Self::FromBitsField(inner) => write!(f, "{}", inner),
            Self::ArrayReverse(inner) => write!(f, "{}", inner),
            Self::ArrayTruncate(inner) => write!(f, "{}", inner),
            Self::ArrayPad(inner) => write!(f, "{}", inner),
        }
    }
}
