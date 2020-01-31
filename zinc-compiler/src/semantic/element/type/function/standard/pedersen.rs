//!
//! The semantic analyzer standard library `pedersen` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
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

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::CryptoPedersen
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, StandardLibraryFunctionError> {
        match inputs.get(0) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, _) => Ok(self.return_type.deref().to_owned()),
                (r#type, size) => Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    "[bool; {N}]".to_owned(),
                    format!("[{}; {}]", r#type, size),
                )),
            },
            Some(r#type) => Err(StandardLibraryFunctionError::ArgumentType(
                self.identifier,
                "[bool; {N}]".to_owned(),
                r#type.to_string(),
            )),
            None => Err(StandardLibraryFunctionError::ArgumentCount(
                self.identifier,
                self.arguments_count(),
                inputs.len(),
            )),
        }
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
