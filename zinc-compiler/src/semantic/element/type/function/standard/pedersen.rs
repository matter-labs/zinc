//!
//! The semantic analyzer standard library `pedersen` function type element.
//!

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::Type;

#[derive(Debug, Clone)]
pub struct PedersenStandardLibraryFunction {
    pub identifier: &'static str,
    pub return_type: Box<Type>,
}

impl PedersenStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "pedersen",
            return_type: Box::new(Type::new_tuple(vec![Type::new_field(), Type::new_field()])),
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::CryptoPedersen
    }
}

impl fmt::Display for PedersenStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(preimage: [bool: N]) -> {}",
            self.identifier, self.return_type,
        )
    }
}
