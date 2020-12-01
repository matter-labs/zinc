//!
//! The semantic analyzer standard library function element.
//!

#[cfg(test)]
mod tests;

pub mod array_pad;
pub mod array_reverse;
pub mod array_truncate;
pub mod collections_mtreemap_contains;
pub mod collections_mtreemap_get;
pub mod collections_mtreemap_insert;
pub mod collections_mtreemap_remove;
pub mod convert_from_bits_field;
pub mod convert_from_bits_signed;
pub mod convert_from_bits_unsigned;
pub mod convert_to_bits;
pub mod crypto_pedersen;
pub mod crypto_schnorr_signature_verify;
pub mod crypto_sha256;
pub mod ff_invert;

use std::fmt;

use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;

use self::array_pad::Function as ArrayPadFunction;
use self::array_reverse::Function as ArrayReverseFunction;
use self::array_truncate::Function as ArrayTruncateFunction;
use self::collections_mtreemap_contains::Function as MTreeMapContainsFunction;
use self::collections_mtreemap_get::Function as MTreeMapGetFunction;
use self::collections_mtreemap_insert::Function as MTreeMapInsertFunction;
use self::collections_mtreemap_remove::Function as MTreeMapRemoveFunction;
use self::convert_from_bits_field::Function as FromBitsFieldFunction;
use self::convert_from_bits_signed::Function as FromBitsSignedFunction;
use self::convert_from_bits_unsigned::Function as FromBitsUnsignedFunction;
use self::convert_to_bits::Function as ToBitsFunction;
use self::crypto_pedersen::Function as PedersenFunction;
use self::crypto_schnorr_signature_verify::Function as SchnorrSignatureVerifyFunction;
use self::crypto_sha256::Function as Sha256Function;
use self::ff_invert::Function as FfInvertFunction;

///
/// The semantic analyzer standard library function element.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `std::crypto::sha256` function variant.
    CryptoSha256(Sha256Function),
    /// The `std::crypto::pedersen` function variant.
    CryptoPedersen(PedersenFunction),
    /// The `std::crypto::schnorr::Signature::verify` function variant.
    CryptoSchnorrSignatureVerify(SchnorrSignatureVerifyFunction),

    /// The `std::convert::to_bits` function variant.
    ConvertToBits(ToBitsFunction),
    /// The `std::convert::from_bits_unsigned` function variant.
    ConvertFromBitsUnsigned(FromBitsUnsignedFunction),
    /// The `std::convert::from_bits_signed` function variant.
    ConvertFromBitsSigned(FromBitsSignedFunction),
    /// The `std::convert::from_bits_field` function variant.
    ConvertFromBitsField(FromBitsFieldFunction),

    /// The `std::array::reverse` function variant.
    ArrayReverse(ArrayReverseFunction),
    /// The `std::array::truncate` function variant.
    ArrayTruncate(ArrayTruncateFunction),
    /// The `std::array::pad` function variant.
    ArrayPad(ArrayPadFunction),

    /// The `std::ff::invert` function variant.
    FfInvert(FfInvertFunction),

    /// The `std::collections::MTreeMap::get` function variant.
    CollectionsMTreeMapGet(MTreeMapGetFunction),
    /// The `std::collections::MTreeMap::contains` function variant.
    CollectionsMTreeMapContains(MTreeMapContainsFunction),
    /// The `std::collections::MTreeMap::insert` function variant.
    CollectionsMTreeMapInsert(MTreeMapInsertFunction),
    /// The `std::collections::MTreeMap::remove` function variant.
    CollectionsMTreeMapRemove(MTreeMapRemoveFunction),
}

impl Function {
    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, location: Location, argument_list: ArgumentList) -> Result<Type, Error> {
        match self {
            Self::CryptoSha256(inner) => inner.call(location, argument_list),
            Self::CryptoPedersen(inner) => inner.call(location, argument_list),
            Self::CryptoSchnorrSignatureVerify(inner) => inner.call(location, argument_list),

            Self::ConvertToBits(inner) => inner.call(location, argument_list),
            Self::ConvertFromBitsUnsigned(inner) => inner.call(location, argument_list),
            Self::ConvertFromBitsSigned(inner) => inner.call(location, argument_list),
            Self::ConvertFromBitsField(inner) => inner.call(location, argument_list),

            Self::ArrayReverse(inner) => inner.call(location, argument_list),
            Self::ArrayTruncate(inner) => inner.call(location, argument_list),
            Self::ArrayPad(inner) => inner.call(location, argument_list),

            Self::FfInvert(inner) => inner.call(location, argument_list),

            Self::CollectionsMTreeMapGet(inner) => inner.call(location, argument_list),
            Self::CollectionsMTreeMapContains(inner) => inner.call(location, argument_list),
            Self::CollectionsMTreeMapInsert(inner) => inner.call(location, argument_list),
            Self::CollectionsMTreeMapRemove(inner) => inner.call(location, argument_list),
        }
    }

    ///
    /// Returns the function identifier, which is known at compile time.
    ///
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

            Self::CollectionsMTreeMapGet(inner) => inner.identifier,
            Self::CollectionsMTreeMapContains(inner) => inner.identifier,
            Self::CollectionsMTreeMapInsert(inner) => inner.identifier,
            Self::CollectionsMTreeMapRemove(inner) => inner.identifier,
        }
    }

    ///
    /// The unique standard library function identifier.
    ///
    pub fn library_identifier(&self) -> LibraryFunctionIdentifier {
        match self {
            Self::CryptoSha256(inner) => inner.library_identifier,
            Self::CryptoPedersen(inner) => inner.library_identifier,
            Self::CryptoSchnorrSignatureVerify(inner) => inner.library_identifier,

            Self::ConvertToBits(inner) => inner.library_identifier,
            Self::ConvertFromBitsUnsigned(inner) => inner.library_identifier,
            Self::ConvertFromBitsSigned(inner) => inner.library_identifier,
            Self::ConvertFromBitsField(inner) => inner.library_identifier,

            Self::ArrayReverse(inner) => inner.library_identifier,
            Self::ArrayTruncate(inner) => inner.library_identifier,
            Self::ArrayPad(inner) => inner.library_identifier,

            Self::FfInvert(inner) => inner.library_identifier,

            Self::CollectionsMTreeMapGet(inner) => inner.library_identifier,
            Self::CollectionsMTreeMapContains(inner) => inner.library_identifier,
            Self::CollectionsMTreeMapInsert(inner) => inner.library_identifier,
            Self::CollectionsMTreeMapRemove(inner) => inner.library_identifier,
        }
    }

    ///
    /// Whether the function must be called from mutable context.
    ///
    pub fn is_mutable(&self) -> bool {
        match self {
            Self::CryptoSha256(_) => false,
            Self::CryptoPedersen(_) => false,
            Self::CryptoSchnorrSignatureVerify(_) => false,

            Self::ConvertToBits(_) => false,
            Self::ConvertFromBitsUnsigned(_) => false,
            Self::ConvertFromBitsSigned(_) => false,
            Self::ConvertFromBitsField(_) => false,

            Self::ArrayReverse(_) => false,
            Self::ArrayTruncate(_) => false,
            Self::ArrayPad(_) => false,

            Self::FfInvert(_) => false,

            Self::CollectionsMTreeMapGet(_) => false,
            Self::CollectionsMTreeMapContains(_) => false,
            Self::CollectionsMTreeMapInsert(_) => true,
            Self::CollectionsMTreeMapRemove(_) => true,
        }
    }

    ///
    /// Sets the function call location in the code.
    ///
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

            Self::CollectionsMTreeMapGet(inner) => inner.location = Some(location),
            Self::CollectionsMTreeMapContains(inner) => inner.location = Some(location),
            Self::CollectionsMTreeMapInsert(inner) => inner.location = Some(location),
            Self::CollectionsMTreeMapRemove(inner) => inner.location = Some(location),
        }
    }

    ///
    /// Returns the location of the function call.
    ///
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

            Self::CollectionsMTreeMapGet(inner) => inner.location,
            Self::CollectionsMTreeMapContains(inner) => inner.location,
            Self::CollectionsMTreeMapInsert(inner) => inner.location,
            Self::CollectionsMTreeMapRemove(inner) => inner.location,
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

            Self::CollectionsMTreeMapGet(inner) => write!(f, "{}", inner),
            Self::CollectionsMTreeMapContains(inner) => write!(f, "{}", inner),
            Self::CollectionsMTreeMapInsert(inner) => write!(f, "{}", inner),
            Self::CollectionsMTreeMapRemove(inner) => write!(f, "{}", inner),
        }
    }
}
