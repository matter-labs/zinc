use enum_primitive_derive::Primitive;
use failure::_core::fmt::{Error, Formatter};
use std::fmt::{Debug, Display};

/// Built-in function identifier.
#[derive(Primitive, Debug, PartialEq, Clone, Copy)]
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
