//!
//! The semantic analyzer standard library function type element.
//!

mod from_bits;
mod pedersen;
mod sha256;
mod to_bits;

pub use self::from_bits::FromBitsStandardLibraryFunction;
pub use self::pedersen::PedersenStandardLibraryFunction;
pub use self::sha256::Sha256StandardLibraryFunction;
pub use self::to_bits::ToBitsStandardLibraryFunction;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

#[derive(Debug, Clone)]
pub enum StandardLibraryFunction {
    Sha256(Sha256StandardLibraryFunction),
    Pedersen(PedersenStandardLibraryFunction),
    FromBits(FromBitsStandardLibraryFunction),
    ToBits(ToBitsStandardLibraryFunction),
}

impl StandardLibraryFunction {
    pub fn new_sha256() -> Self {
        Self::Sha256(Sha256StandardLibraryFunction::new())
    }

    pub fn new_pedersen() -> Self {
        Self::Pedersen(PedersenStandardLibraryFunction::new())
    }

    pub fn new_from_bits() -> Self {
        Self::FromBits(FromBitsStandardLibraryFunction::new())
    }

    pub fn new_to_bits() -> Self {
        Self::ToBits(ToBitsStandardLibraryFunction::new())
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Sha256(inner) => inner.identifier,
            Self::Pedersen(inner) => inner.identifier,
            Self::FromBits(inner) => inner.identifier,
            Self::ToBits(inner) => inner.identifier,
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::Sha256(_) => Sha256StandardLibraryFunction::builtin_identifier(),
            Self::Pedersen(_) => PedersenStandardLibraryFunction::builtin_identifier(),
            Self::FromBits(_) => FromBitsStandardLibraryFunction::builtin_identifier(),
            Self::ToBits(_) => ToBitsStandardLibraryFunction::builtin_identifier(),
        }
    }
}

impl fmt::Display for StandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256(inner) => write!(f, "{}", inner),
            Self::Pedersen(inner) => write!(f, "{}", inner),
            Self::FromBits(inner) => write!(f, "{}", inner),
            Self::ToBits(inner) => write!(f, "{}", inner),
        }
    }
}
