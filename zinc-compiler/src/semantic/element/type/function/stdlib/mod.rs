//!
//! The semantic analyzer standard library function type element.
//!

pub mod array_pad;
pub mod array_reverse;
pub mod array_truncate;
pub mod convert_from_bits_field;
pub mod convert_from_bits_signed;
pub mod convert_from_bits_unsigned;
pub mod convert_to_bits;
pub mod crypto_pedersen;
pub mod crypto_schnorr_verify;
pub mod crypto_sha256;
pub mod error;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use self::array_pad::Function as ArrayPadFunction;
use self::array_reverse::Function as ArrayReverseFunction;
use self::array_truncate::Function as ArrayTruncateFunction;
use self::convert_from_bits_field::Function as FromBitsFieldFunction;
use self::convert_from_bits_signed::Function as FromBitsSignedFunction;
use self::convert_from_bits_unsigned::Function as FromBitsUnsignedFunction;
use self::convert_to_bits::Function as ToBitsFunction;
use self::crypto_pedersen::Function as PedersenFunction;
use self::crypto_schnorr_verify::Function as SchnorrVerifyFunction;
use self::crypto_sha256::Function as Sha256Function;

#[derive(Debug, Clone)]
pub enum Function {
    CryptoSha256(Sha256Function),
    CryptoPedersen(PedersenFunction),
    CryptoSchnorrVerify(SchnorrVerifyFunction),

    ConvertToBits(ToBitsFunction),
    ConvertFromBitsUnsigned(FromBitsUnsignedFunction),
    ConvertFromBitsSigned(FromBitsSignedFunction),
    ConvertFromBitsField(FromBitsFieldFunction),

    ArrayReverse(ArrayReverseFunction),
    ArrayTruncate(ArrayTruncateFunction),
    ArrayPad(ArrayPadFunction),
}

impl Function {
    pub fn new(identifier: BuiltinIdentifier) -> Self {
        match identifier {
            BuiltinIdentifier::CryptoSha256 => Self::CryptoSha256(Sha256Function::new()),
            BuiltinIdentifier::CryptoPedersen => Self::CryptoPedersen(PedersenFunction::new()),
            BuiltinIdentifier::CryptoSchnorrVerify => {
                Self::CryptoSchnorrVerify(SchnorrVerifyFunction::new())
            }

            BuiltinIdentifier::ToBits => Self::ConvertToBits(ToBitsFunction::new()),
            BuiltinIdentifier::UnsignedFromBits => {
                Self::ConvertFromBitsUnsigned(FromBitsUnsignedFunction::new())
            }
            BuiltinIdentifier::SignedFromBits => {
                Self::ConvertFromBitsSigned(FromBitsSignedFunction::new())
            }
            BuiltinIdentifier::FieldFromBits => {
                Self::ConvertFromBitsField(FromBitsFieldFunction::new())
            }

            BuiltinIdentifier::ArrayReverse => Self::ArrayReverse(ArrayReverseFunction::new()),
            BuiltinIdentifier::ArrayTruncate => Self::ArrayTruncate(ArrayTruncateFunction::new()),
            BuiltinIdentifier::ArrayPad => Self::ArrayPad(ArrayPadFunction::new()),
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::CryptoSha256(inner) => inner.identifier(),
            Self::CryptoPedersen(inner) => inner.identifier(),
            Self::CryptoSchnorrVerify(inner) => inner.identifier(),

            Self::ConvertToBits(inner) => inner.identifier(),
            Self::ConvertFromBitsUnsigned(inner) => inner.identifier(),
            Self::ConvertFromBitsSigned(inner) => inner.identifier(),
            Self::ConvertFromBitsField(inner) => inner.identifier(),

            Self::ArrayReverse(inner) => inner.identifier(),
            Self::ArrayTruncate(inner) => inner.identifier(),
            Self::ArrayPad(inner) => inner.identifier(),
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::CryptoSha256(inner) => inner.builtin_identifier(),
            Self::CryptoPedersen(inner) => inner.builtin_identifier(),
            Self::CryptoSchnorrVerify(inner) => inner.builtin_identifier(),

            Self::ConvertToBits(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsUnsigned(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsSigned(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsField(inner) => inner.builtin_identifier(),

            Self::ArrayReverse(inner) => inner.builtin_identifier(),
            Self::ArrayTruncate(inner) => inner.builtin_identifier(),
            Self::ArrayPad(inner) => inner.builtin_identifier(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CryptoSha256(inner) => write!(f, "{}", inner),
            Self::CryptoPedersen(inner) => write!(f, "{}", inner),
            Self::CryptoSchnorrVerify(inner) => write!(f, "{}", inner),

            Self::ConvertToBits(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsUnsigned(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsSigned(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsField(inner) => write!(f, "{}", inner),

            Self::ArrayReverse(inner) => write!(f, "{}", inner),
            Self::ArrayTruncate(inner) => write!(f, "{}", inner),
            Self::ArrayPad(inner) => write!(f, "{}", inner),
        }
    }
}
