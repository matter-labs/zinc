//!
//! The semantic analyzer standard library `from_bits_field` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct FromBitsFieldStandardLibraryFunction {
    identifier: &'static str,
}

impl FromBitsFieldStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "from_bits_field",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::FieldFromBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, StandardLibraryFunctionError> {
        match inputs.get(0) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, crate::BITLENGTH_FIELD) => Ok(Type::new_field()),
                (r#type, size) => Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    format!("[bool; {}]", crate::BITLENGTH_FIELD),
                    format!("[{}; {}]", r#type, size),
                )),
            },
            Some(r#type) => Err(StandardLibraryFunctionError::ArgumentType(
                self.identifier,
                format!("[bool; {}]", crate::BITLENGTH_FIELD),
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

impl fmt::Display for FromBitsFieldStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(bits: [bool; {}]) -> field",
            self.identifier,
            crate::BITLENGTH_FIELD,
        )
    }
}
