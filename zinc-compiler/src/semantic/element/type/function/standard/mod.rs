//!
//! The semantic analyzer standard library function type element.
//!

mod array_pad;
mod array_reverse;
mod array_truncate;
mod error;
mod from_bits_field;
mod from_bits_signed;
mod from_bits_unsigned;
mod pedersen;
mod sha256;
mod to_bits;

pub use self::array_pad::ArrayPadStandardLibraryFunction;
pub use self::array_reverse::ArrayReverseStandardLibraryFunction;
pub use self::array_truncate::ArrayTruncateStandardLibraryFunction;
pub use self::error::Error;
pub use self::from_bits_field::FromBitsFieldStandardLibraryFunction;
pub use self::from_bits_signed::FromBitsSignedStandardLibraryFunction;
pub use self::from_bits_unsigned::FromBitsUnsignedStandardLibraryFunction;
pub use self::pedersen::PedersenStandardLibraryFunction;
pub use self::sha256::Sha256StandardLibraryFunction;
pub use self::to_bits::ToBitsStandardLibraryFunction;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

#[derive(Debug, Clone)]
pub enum StandardLibraryFunction {
    Sha256(Sha256StandardLibraryFunction),
    Pedersen(PedersenStandardLibraryFunction),
    ToBits(ToBitsStandardLibraryFunction),
    FromBitsUnsigned(FromBitsUnsignedStandardLibraryFunction),
    FromBitsSigned(FromBitsSignedStandardLibraryFunction),
    FromBitsField(FromBitsFieldStandardLibraryFunction),
    ArrayReverse(ArrayReverseStandardLibraryFunction),
    ArrayTruncate(ArrayTruncateStandardLibraryFunction),
    ArrayPad(ArrayPadStandardLibraryFunction),
}

impl StandardLibraryFunction {
    pub fn new(identifier: BuiltinIdentifier) -> Self {
        match identifier {
            BuiltinIdentifier::CryptoSha256 => Self::Sha256(Sha256StandardLibraryFunction::new()),
            BuiltinIdentifier::CryptoPedersen => {
                Self::Pedersen(PedersenStandardLibraryFunction::new())
            }
            BuiltinIdentifier::ToBits => Self::ToBits(ToBitsStandardLibraryFunction::new()),
            BuiltinIdentifier::UnsignedFromBits => {
                Self::FromBitsUnsigned(FromBitsUnsignedStandardLibraryFunction::new())
            }
            BuiltinIdentifier::SignedFromBits => {
                Self::FromBitsSigned(FromBitsSignedStandardLibraryFunction::new())
            }
            BuiltinIdentifier::FieldFromBits => {
                Self::FromBitsField(FromBitsFieldStandardLibraryFunction::new())
            }
            BuiltinIdentifier::ArrayReverse => {
                Self::ArrayReverse(ArrayReverseStandardLibraryFunction::new())
            }
            BuiltinIdentifier::ArrayTruncate => {
                Self::ArrayTruncate(ArrayTruncateStandardLibraryFunction::new())
            }
            BuiltinIdentifier::ArrayPad => Self::ArrayPad(ArrayPadStandardLibraryFunction::new()),
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Sha256(inner) => inner.identifier,
            Self::Pedersen(inner) => inner.identifier,
            Self::ToBits(inner) => inner.identifier,
            Self::FromBitsUnsigned(inner) => inner.identifier,
            Self::FromBitsSigned(inner) => inner.identifier,
            Self::FromBitsField(inner) => inner.identifier,
            Self::ArrayReverse(inner) => inner.identifier,
            Self::ArrayTruncate(inner) => inner.identifier,
            Self::ArrayPad(inner) => inner.identifier,
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::Sha256(_) => Sha256StandardLibraryFunction::builtin_identifier(),
            Self::Pedersen(_) => PedersenStandardLibraryFunction::builtin_identifier(),
            Self::ToBits(_) => ToBitsStandardLibraryFunction::builtin_identifier(),
            Self::FromBitsUnsigned(_) => {
                FromBitsUnsignedStandardLibraryFunction::builtin_identifier()
            }
            Self::FromBitsSigned(_) => FromBitsSignedStandardLibraryFunction::builtin_identifier(),
            Self::FromBitsField(_) => FromBitsFieldStandardLibraryFunction::builtin_identifier(),
            Self::ArrayReverse(_) => FromBitsFieldStandardLibraryFunction::builtin_identifier(),
            Self::ArrayTruncate(_) => FromBitsFieldStandardLibraryFunction::builtin_identifier(),
            Self::ArrayPad(_) => FromBitsFieldStandardLibraryFunction::builtin_identifier(),
        }
    }
}

impl fmt::Display for StandardLibraryFunction {
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
