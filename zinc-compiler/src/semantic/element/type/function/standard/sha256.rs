//!
//! The semantic analyzer standard library `sha256` function type element.
//!

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct Sha256StandardLibraryFunction {
    pub identifier: &'static str,
    pub return_type: Box<Type>,
}

impl Sha256StandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "sha256",
            return_type: Box::new(Type::new_array(
                Type::new_boolean(),
                crate::SHA256_HASH_SIZE_BITS,
            )),
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::CryptoSha256
    }

    pub fn arguments_count(&self) -> usize {
        1
    }
}

impl fmt::Display for Sha256StandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(preimage: [bool: N]) -> {}",
            self.identifier, self.return_type,
        )
    }
}
