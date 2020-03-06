use enum_primitive_derive::Primitive;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Error, Formatter};

/// Built-in function identifier.
#[derive(Primitive, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BuiltinIdentifier {
    CryptoSha256 = 1,
    CryptoPedersen = 2,
    ToBits = 3,
    UnsignedFromBits = 4,
    SignedFromBits = 5,
    FieldFromBits = 6,
    ArrayReverse = 7,
    ArrayTruncate = 8,
    ArrayPad = 9,
    CryptoSchnorrSignatureVerify = 11,
}

impl Display for BuiltinIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name = match self {
            BuiltinIdentifier::CryptoSha256 => "CryptoSha256",
            BuiltinIdentifier::CryptoPedersen => "CryptoPedersen",
            BuiltinIdentifier::ToBits => "ToBits",
            BuiltinIdentifier::UnsignedFromBits => "UnsignedFromBits",
            BuiltinIdentifier::SignedFromBits => "SignedFromBits",
            BuiltinIdentifier::FieldFromBits => "FieldFromBits",
            BuiltinIdentifier::ArrayReverse => "ArrayReverse",
            BuiltinIdentifier::ArrayTruncate => "ArrayTruncate",
            BuiltinIdentifier::ArrayPad => "ArrayPad",
            BuiltinIdentifier::CryptoSchnorrSignatureVerify => "CryptoSchnorrVerify",
        };
        f.write_str(name)
    }
}
