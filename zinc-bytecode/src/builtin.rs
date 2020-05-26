use std::fmt::Debug;

use serde_derive::Deserialize;
use serde_derive::Serialize;

/// Built-in function identifier.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BuiltinIdentifier {
    CryptoSha256,
    CryptoPedersen,
    ToBits,
    UnsignedFromBits,
    SignedFromBits,
    FieldFromBits,
    ArrayReverse,
    ArrayTruncate,
    ArrayPad,
    CryptoSchnorrSignatureVerify,
    FieldInverse,
}
