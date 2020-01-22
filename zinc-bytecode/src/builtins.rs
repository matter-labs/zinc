use enum_primitive_derive::Primitive;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Error, Formatter};

/// Built-in function identifier.
#[derive(Primitive, Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BuiltinIdentifier {
    CryptoSha256 = 1,
    CryptoPedersen = 2,
}

impl Display for BuiltinIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let name = match self {
            BuiltinIdentifier::CryptoSha256 => "CryptoSha256",
            BuiltinIdentifier::CryptoPedersen => "CryptoPedersen",
        };
        f.write_str(name)
    }
}
