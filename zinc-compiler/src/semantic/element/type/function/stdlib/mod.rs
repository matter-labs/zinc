//!
//! The semantic analyzer standard library function element.
//!

mod tests;

pub mod array_pad;
pub mod array_reverse;
pub mod array_truncate;
pub mod convert_from_bits_field;
pub mod convert_from_bits_signed;
pub mod convert_from_bits_unsigned;
pub mod convert_to_bits;
pub mod crypto_blake2s;
pub mod crypto_blake2s_multi_input;
pub mod crypto_pedersen;
pub mod crypto_schnorr_signature_verify;
pub mod crypto_sha256;
pub mod error;
pub mod ff_invert;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

use self::array_pad::Function as ArrayPadFunction;
use self::array_reverse::Function as ArrayReverseFunction;
use self::array_truncate::Function as ArrayTruncateFunction;
use self::convert_from_bits_field::Function as FromBitsFieldFunction;
use self::convert_from_bits_signed::Function as FromBitsSignedFunction;
use self::convert_from_bits_unsigned::Function as FromBitsUnsignedFunction;
use self::convert_to_bits::Function as ToBitsFunction;
use self::crypto_blake2s::Function as Blake2sFunction;
use self::crypto_blake2s_multi_input::Function as Blake2sMultiInputFunction;
use self::crypto_pedersen::Function as PedersenFunction;
use self::crypto_schnorr_signature_verify::Function as SchnorrSignatureVerifyFunction;
use self::crypto_sha256::Function as Sha256Function;
use self::ff_invert::Function as FfInvertFunction;

#[derive(Debug, Clone)]
pub enum Function {
    CryptoSha256(Sha256Function),
    CryptoPedersen(PedersenFunction),
    CryptoSchnorrSignatureVerify(SchnorrSignatureVerifyFunction),
    CryptoBlake2s(Blake2sFunction),
    CryptoBlake2sMultiInput(Blake2sMultiInputFunction),

    ConvertToBits(ToBitsFunction),
    ConvertFromBitsUnsigned(FromBitsUnsignedFunction),
    ConvertFromBitsSigned(FromBitsSignedFunction),
    ConvertFromBitsField(FromBitsFieldFunction),

    ArrayReverse(ArrayReverseFunction),
    ArrayTruncate(ArrayTruncateFunction),
    ArrayPad(ArrayPadFunction),

    FfInvert(FfInvertFunction),
}

impl Function {
    pub fn new(identifier: BuiltinIdentifier) -> Self {
        match identifier {
            BuiltinIdentifier::CryptoSha256 => Self::CryptoSha256(Sha256Function::new(identifier)),
            BuiltinIdentifier::CryptoPedersen => {
                Self::CryptoPedersen(PedersenFunction::new(identifier))
            }
            BuiltinIdentifier::CryptoSchnorrSignatureVerify => {
                Self::CryptoSchnorrSignatureVerify(SchnorrSignatureVerifyFunction::new(identifier))
            }
            BuiltinIdentifier::CryptoBlake2s => {
                Self::CryptoBlake2s(Blake2sFunction::new(identifier))
            }
            BuiltinIdentifier::CryptoBlake2sMultiInput => {
                Self::CryptoBlake2sMultiInput(Blake2sMultiInputFunction::new(identifier))
            }

            BuiltinIdentifier::ToBits => Self::ConvertToBits(ToBitsFunction::new(identifier)),
            BuiltinIdentifier::UnsignedFromBits => {
                Self::ConvertFromBitsUnsigned(FromBitsUnsignedFunction::new(identifier))
            }
            BuiltinIdentifier::SignedFromBits => {
                Self::ConvertFromBitsSigned(FromBitsSignedFunction::new(identifier))
            }
            BuiltinIdentifier::FieldFromBits => {
                Self::ConvertFromBitsField(FromBitsFieldFunction::new(identifier))
            }

            BuiltinIdentifier::ArrayReverse => {
                Self::ArrayReverse(ArrayReverseFunction::new(identifier))
            }
            BuiltinIdentifier::ArrayTruncate => {
                Self::ArrayTruncate(ArrayTruncateFunction::new(identifier))
            }
            BuiltinIdentifier::ArrayPad => Self::ArrayPad(ArrayPadFunction::new(identifier)),

            BuiltinIdentifier::FieldInverse => Self::FfInvert(FfInvertFunction::new(identifier)),
        }
    }

    pub fn call(self, elements: Vec<Element>) -> Result<Type, Error> {
        match self {
            Self::CryptoSha256(inner) => inner.call(elements),
            Self::CryptoPedersen(inner) => inner.call(elements),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.call(elements),
            Self::CryptoBlake2s(inner) => inner.call(elements),
            Self::CryptoBlake2sMultiInput(inner) => inner.call(elements),

            Self::ConvertToBits(inner) => inner.call(elements),
            Self::ConvertFromBitsUnsigned(inner) => inner.call(elements),
            Self::ConvertFromBitsSigned(inner) => inner.call(elements),
            Self::ConvertFromBitsField(inner) => inner.call(elements),

            Self::ArrayReverse(inner) => inner.call(elements),
            Self::ArrayTruncate(inner) => inner.call(elements),
            Self::ArrayPad(inner) => inner.call(elements),

            Self::FfInvert(inner) => inner.call(elements),
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::CryptoSha256(inner) => inner.identifier(),
            Self::CryptoPedersen(inner) => inner.identifier(),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.identifier(),
            Self::CryptoBlake2s(inner) => inner.identifier(),
            Self::CryptoBlake2sMultiInput(inner) => inner.identifier(),

            Self::ConvertToBits(inner) => inner.identifier(),
            Self::ConvertFromBitsUnsigned(inner) => inner.identifier(),
            Self::ConvertFromBitsSigned(inner) => inner.identifier(),
            Self::ConvertFromBitsField(inner) => inner.identifier(),

            Self::ArrayReverse(inner) => inner.identifier(),
            Self::ArrayTruncate(inner) => inner.identifier(),
            Self::ArrayPad(inner) => inner.identifier(),

            Self::FfInvert(inner) => inner.identifier(),
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::CryptoSha256(inner) => inner.builtin_identifier(),
            Self::CryptoPedersen(inner) => inner.builtin_identifier(),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.builtin_identifier(),
            Self::CryptoBlake2s(inner) => inner.builtin_identifier(),
            Self::CryptoBlake2sMultiInput(inner) => inner.builtin_identifier(),

            Self::ConvertToBits(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsUnsigned(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsSigned(inner) => inner.builtin_identifier(),
            Self::ConvertFromBitsField(inner) => inner.builtin_identifier(),

            Self::ArrayReverse(inner) => inner.builtin_identifier(),
            Self::ArrayTruncate(inner) => inner.builtin_identifier(),
            Self::ArrayPad(inner) => inner.builtin_identifier(),

            Self::FfInvert(inner) => inner.builtin_identifier(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CryptoSha256(inner) => write!(f, "{}", inner),
            Self::CryptoPedersen(inner) => write!(f, "{}", inner),
            Self::CryptoSchnorrSignatureVerify(inner) => write!(f, "{}", inner),
            Self::CryptoBlake2s(inner) => write!(f, "{}", inner),
            Self::CryptoBlake2sMultiInput(inner) => write!(f, "{}", inner),

            Self::ConvertToBits(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsUnsigned(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsSigned(inner) => write!(f, "{}", inner),
            Self::ConvertFromBitsField(inner) => write!(f, "{}", inner),

            Self::ArrayReverse(inner) => write!(f, "{}", inner),
            Self::ArrayTruncate(inner) => write!(f, "{}", inner),
            Self::ArrayPad(inner) => write!(f, "{}", inner),

            Self::FfInvert(inner) => write!(f, "{}", inner),
        }
    }
}
