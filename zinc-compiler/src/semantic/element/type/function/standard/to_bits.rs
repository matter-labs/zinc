//!
//! The semantic analyzer standard library `to_bits` function type element.
//!

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct ToBitsStandardLibraryFunction {
    pub identifier: &'static str,
}

impl ToBitsStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "to_bits",
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::ToBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, StandardLibraryFunctionError> {
        match inputs.get(0) {
            Some(Type::Boolean) => Ok(Type::new_array(
                Type::new_boolean(),
                crate::BITLENGTH_BOOLEAN,
            )),
            Some(Type::IntegerUnsigned { bitlength }) => {
                Ok(Type::new_array(Type::new_boolean(), *bitlength))
            }
            Some(Type::IntegerSigned { bitlength }) => {
                Ok(Type::new_array(Type::new_boolean(), *bitlength))
            }
            Some(Type::Field) => Ok(Type::new_array(Type::new_boolean(), crate::BITLENGTH_FIELD)),
            Some(r#type) => Err(StandardLibraryFunctionError::ArgumentType(
                self.identifier,
                "integer".to_owned(),
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

impl fmt::Display for ToBitsStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fn std::{}(value: field) -> [bool: N]", self.identifier,)
    }
}
