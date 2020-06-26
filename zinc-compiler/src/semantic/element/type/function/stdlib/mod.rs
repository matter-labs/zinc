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
pub mod crypto_pedersen;
pub mod crypto_schnorr_signature_verify;
pub mod crypto_sha256;
pub mod error;
pub mod ff_invert;

use std::fmt;

use zinc_bytecode::FunctionIdentifier;

use crate::lexical::token::location::Location;
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
use self::crypto_pedersen::Function as PedersenFunction;
use self::crypto_schnorr_signature_verify::Function as SchnorrSignatureVerifyFunction;
use self::crypto_sha256::Function as Sha256Function;
use self::ff_invert::Function as FfInvertFunction;

#[derive(Debug, Clone)]
pub enum Function {
    CryptoSha256(Sha256Function),
    CryptoPedersen(PedersenFunction),
    CryptoSchnorrSignatureVerify(SchnorrSignatureVerifyFunction),

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
    pub fn new(identifier: FunctionIdentifier) -> Self {
        match identifier {
            FunctionIdentifier::CryptoSha256 => Self::CryptoSha256(Sha256Function::new(identifier)),
            FunctionIdentifier::CryptoPedersen => {
                Self::CryptoPedersen(PedersenFunction::new(identifier))
            }
            FunctionIdentifier::CryptoSchnorrSignatureVerify => {
                Self::CryptoSchnorrSignatureVerify(SchnorrSignatureVerifyFunction::new(identifier))
            }

            FunctionIdentifier::ConvertToBits => {
                Self::ConvertToBits(ToBitsFunction::new(identifier))
            }
            FunctionIdentifier::ConvertFromBitsUnsigned => {
                Self::ConvertFromBitsUnsigned(FromBitsUnsignedFunction::new(identifier))
            }
            FunctionIdentifier::ConvertFromBitsSigned => {
                Self::ConvertFromBitsSigned(FromBitsSignedFunction::new(identifier))
            }
            FunctionIdentifier::ConvertFromBitsField => {
                Self::ConvertFromBitsField(FromBitsFieldFunction::new(identifier))
            }

            FunctionIdentifier::ArrayReverse => {
                Self::ArrayReverse(ArrayReverseFunction::new(identifier))
            }
            FunctionIdentifier::ArrayTruncate => {
                Self::ArrayTruncate(ArrayTruncateFunction::new(identifier))
            }
            FunctionIdentifier::ArrayPad => Self::ArrayPad(ArrayPadFunction::new(identifier)),

            FunctionIdentifier::FieldInverse => Self::FfInvert(FfInvertFunction::new(identifier)),
        }
    }

    pub fn call(self, location: Option<Location>, elements: Vec<Element>) -> Result<Type, Error> {
        match self {
            Self::CryptoSha256(inner) => inner.call(location, elements),
            Self::CryptoPedersen(inner) => inner.call(location, elements),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.call(location, elements),

            Self::ConvertToBits(inner) => inner.call(location, elements),
            Self::ConvertFromBitsUnsigned(inner) => inner.call(location, elements),
            Self::ConvertFromBitsSigned(inner) => inner.call(location, elements),
            Self::ConvertFromBitsField(inner) => inner.call(location, elements),

            Self::ArrayReverse(inner) => inner.call(location, elements),
            Self::ArrayTruncate(inner) => inner.call(location, elements),
            Self::ArrayPad(inner) => inner.call(location, elements),

            Self::FfInvert(inner) => inner.call(location, elements),
        }
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::CryptoSha256(inner) => inner.identifier,
            Self::CryptoPedersen(inner) => inner.identifier,
            Self::CryptoSchnorrSignatureVerify(inner) => inner.identifier,

            Self::ConvertToBits(inner) => inner.identifier,
            Self::ConvertFromBitsUnsigned(inner) => inner.identifier,
            Self::ConvertFromBitsSigned(inner) => inner.identifier,
            Self::ConvertFromBitsField(inner) => inner.identifier,

            Self::ArrayReverse(inner) => inner.identifier,
            Self::ArrayTruncate(inner) => inner.identifier,
            Self::ArrayPad(inner) => inner.identifier,

            Self::FfInvert(inner) => inner.identifier,
        }
    }

    pub fn builtin_identifier(&self) -> FunctionIdentifier {
        match self {
            Self::CryptoSha256(inner) => inner.builtin_identifier,
            Self::CryptoPedersen(inner) => inner.builtin_identifier,
            Self::CryptoSchnorrSignatureVerify(inner) => inner.builtin_identifier,

            Self::ConvertToBits(inner) => inner.builtin_identifier,
            Self::ConvertFromBitsUnsigned(inner) => inner.builtin_identifier,
            Self::ConvertFromBitsSigned(inner) => inner.builtin_identifier,
            Self::ConvertFromBitsField(inner) => inner.builtin_identifier,

            Self::ArrayReverse(inner) => inner.builtin_identifier,
            Self::ArrayTruncate(inner) => inner.builtin_identifier,
            Self::ArrayPad(inner) => inner.builtin_identifier,

            Self::FfInvert(inner) => inner.builtin_identifier,
        }
    }

    pub fn set_location(&mut self, location: Location) {
        match self {
            Self::CryptoSha256(inner) => inner.location = Some(location),
            Self::CryptoPedersen(inner) => inner.location = Some(location),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.location = Some(location),

            Self::ConvertToBits(inner) => inner.location = Some(location),
            Self::ConvertFromBitsUnsigned(inner) => inner.location = Some(location),
            Self::ConvertFromBitsSigned(inner) => inner.location = Some(location),
            Self::ConvertFromBitsField(inner) => inner.location = Some(location),

            Self::ArrayReverse(inner) => inner.location = Some(location),
            Self::ArrayTruncate(inner) => inner.location = Some(location),
            Self::ArrayPad(inner) => inner.location = Some(location),

            Self::FfInvert(inner) => inner.location = Some(location),
        }
    }

    pub fn location(&self) -> Option<Location> {
        match self {
            Self::CryptoSha256(inner) => inner.location,
            Self::CryptoPedersen(inner) => inner.location,
            Self::CryptoSchnorrSignatureVerify(inner) => inner.location,

            Self::ConvertToBits(inner) => inner.location,
            Self::ConvertFromBitsUnsigned(inner) => inner.location,
            Self::ConvertFromBitsSigned(inner) => inner.location,
            Self::ConvertFromBitsField(inner) => inner.location,

            Self::ArrayReverse(inner) => inner.location,
            Self::ArrayTruncate(inner) => inner.location,
            Self::ArrayPad(inner) => inner.location,

            Self::FfInvert(inner) => inner.location,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CryptoSha256(inner) => write!(f, "{}", inner),
            Self::CryptoPedersen(inner) => write!(f, "{}", inner),
            Self::CryptoSchnorrSignatureVerify(inner) => write!(f, "{}", inner),

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
