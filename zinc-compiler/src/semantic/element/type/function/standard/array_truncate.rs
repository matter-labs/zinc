//!
//! The semantic analyzer standard library `array_truncate` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::standard::error::Error;
use crate::semantic::element::r#type::Type;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    pub fn new() -> Self {
        Self {
            identifier: "truncate",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::ArrayTruncate
    }

    pub fn arguments_count(&self) -> usize {
        2
    }

    pub fn validate(&self, inputs: &[Type], new_length: usize) -> Result<Type, Error> {
        let (input_array_type, input_array_size) = match inputs.get(0) {
            Some(Type::Array { r#type, size }) if r#type.is_scalar() => {
                (r#type.deref().to_owned(), *size)
            }
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    "[{scalar}; {N}]".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    self.arguments_count(),
                    inputs.len(),
                ))
            }
        };

        match inputs.get(1) {
            Some(new_length) if new_length.is_scalar_unsigned() => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    "{scalar}".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    self.arguments_count(),
                    inputs.len(),
                ))
            }
        }

        if inputs.get(2).is_some() {
            return Err(Error::ArgumentCount(
                self.identifier,
                self.arguments_count(),
                inputs.len(),
            ));
        }

        if new_length > input_array_size {
            return Err(Error::TruncateInvalidLength(input_array_size, new_length));
        }

        Ok(Type::array(input_array_type, new_length))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(array: [{{T}}; {{N}}]) -> [{{T}}; {{N}}]",
            self.identifier,
        )
    }
}
