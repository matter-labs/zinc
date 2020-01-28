//!
//! The semantic analyzer standard library `array_reverse` function type element.
//!

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct ArrayReverseStandardLibraryFunction {
    pub identifier: &'static str,
}

impl ArrayReverseStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "array_reverse",
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::ArrayReverse
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, StandardLibraryFunctionError> {
        match inputs.get(0) {
            Some(array @ Type::Array { .. }) if array.is_scalar() => Ok(array.to_owned()),
            Some(r#type) => Err(StandardLibraryFunctionError::ArgumentType(
                self.identifier,
                "[{scalar}; {N}]".to_owned(),
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

impl fmt::Display for ArrayReverseStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(array: [{{T}}; {{N}}]) -> [{{T}}; {{N}}]",
            self.identifier,
        )
    }
}
