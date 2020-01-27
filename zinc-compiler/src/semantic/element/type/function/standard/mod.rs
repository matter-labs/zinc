//!
//! The semantic analyzer standard library function type element.
//!

mod pedersen;
mod sha256;

pub use self::pedersen::PedersenStandardLibraryFunction;
pub use self::sha256::Sha256StandardLibraryFunction;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

#[derive(Debug, Clone)]
pub enum StandardLibraryFunction {
    Sha256(Sha256StandardLibraryFunction),
    Pedersen(PedersenStandardLibraryFunction),
}

impl StandardLibraryFunction {
    pub fn new_sha256() -> Self {
        Self::Sha256(Sha256StandardLibraryFunction::new())
    }

    pub fn new_pedersen() -> Self {
        Self::Pedersen(PedersenStandardLibraryFunction::new())
    }

    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Sha256(inner) => inner.identifier,
            Self::Pedersen(inner) => inner.identifier,
        }
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        match self {
            Self::Sha256(_) => Sha256StandardLibraryFunction::builtin_identifier(),
            Self::Pedersen(_) => PedersenStandardLibraryFunction::builtin_identifier(),
        }
    }
}

impl fmt::Display for StandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256(inner) => write!(f, "{}", inner),
            Self::Pedersen(inner) => write!(f, "{}", inner),
        }
    }
}
