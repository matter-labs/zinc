use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

/// Built-in function identifier.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BuiltinIdentifier {
    CryptoSha256 = 1,
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
    CryptoBlake2s,
    CryptoBlake2sMultiInput,
}
