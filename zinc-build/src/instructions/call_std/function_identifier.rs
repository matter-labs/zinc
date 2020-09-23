//!
//! The standard library function identifier.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

///
/// The standard library function identifier.
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FunctionIdentifier {
    /// The `std::crypto::sha256` function identifier.
    CryptoSha256,
    /// The `std::crypto::pedersen` function identifier.
    CryptoPedersen,
    /// The `std::crypto::schnorr::Signature::verify` function identifier.
    CryptoSchnorrSignatureVerify,

    /// The `std::convert::to_bits` function identifier.
    ConvertToBits,
    /// The `std::convert::from_bits_unsigned` function identifier.
    ConvertFromBitsUnsigned,
    /// The `std::convert::from_bits_signed` function identifier.
    ConvertFromBitsSigned,
    /// The `std::convert::from_bits_field` function identifier.
    ConvertFromBitsField,

    /// The `std::array::reverse` function identifier.
    ArrayReverse,
    /// The `std::array::truncate` function identifier.
    ArrayTruncate,
    /// The `std::array::pad` function identifier.
    ArrayPad,

    /// The `std::ff::invert` function identifier.
    FieldInverse,

    /// The `std::zksync::transfer` function identifier.
    ZksyncTransfer,
}
