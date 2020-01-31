//!
//! The semantic analyzer standard library `array_pad` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::StandardLibraryFunctionError;
use crate::semantic::Type;

#[derive(Debug, Default, Clone)]
pub struct ArrayPadStandardLibraryFunction {
    pub identifier: &'static str,
}

impl ArrayPadStandardLibraryFunction {
    pub fn new() -> Self {
        Self {
            identifier: "array_pad",
        }
    }

    pub fn builtin_identifier() -> BuiltinIdentifier {
        BuiltinIdentifier::ArrayPad
    }

    pub fn arguments_count(&self) -> usize {
        3
    }

    pub fn validate(
        &self,
        inputs: &[Type],
        new_length: usize,
    ) -> Result<Type, StandardLibraryFunctionError> {
        let (input_array_type, input_array_size) = match inputs.get(0) {
            Some(Type::Array { r#type, size }) if r#type.is_scalar() => {
                (r#type.deref().to_owned(), *size)
            }
            Some(r#type) => {
                return Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    "[{scalar}; {N}]".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(StandardLibraryFunctionError::ArgumentCount(
                    self.identifier,
                    self.arguments_count(),
                    inputs.len(),
                ))
            }
        };

        match inputs.get(1) {
            Some(new_length) if new_length.is_scalar_unsigned() => {}
            Some(r#type) => {
                return Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    "{scalar}".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(StandardLibraryFunctionError::ArgumentCount(
                    self.identifier,
                    self.arguments_count(),
                    inputs.len(),
                ))
            }
        }

        match inputs.get(2) {
            Some(r#type) if r#type.is_scalar() => {}
            Some(r#type) => {
                return Err(StandardLibraryFunctionError::ArgumentType(
                    self.identifier,
                    "{scalar}".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(StandardLibraryFunctionError::ArgumentCount(
                    self.identifier,
                    self.arguments_count(),
                    inputs.len(),
                ))
            }
        }

        if new_length < input_array_size {
            return Err(StandardLibraryFunctionError::PadInvalidLength(
                input_array_size,
                new_length,
            ));
        }

        Ok(Type::new_array(input_array_type, new_length))
    }
}

impl fmt::Display for ArrayPadStandardLibraryFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(array: [{{T}}; {{N}}]) -> [{{T}}; {{N}}]",
            self.identifier,
        )
    }
}
