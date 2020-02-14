//!
//! The semantic analyzer standard library `array_pad` function type element.
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
    const ARGUMENT_INDEX_ARRAY: usize = 0;
    const ARGUMENT_INDEX_NEW_LENGTH: usize = 1;
    const ARGUMENT_INDEX_FILL_VALUE: usize = 2;
    const ARGUMENT_COUNT: usize = 3;

    pub fn new() -> Self {
        Self { identifier: "pad" }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::ArrayPad
    }

    pub fn validate(&self, inputs: &[Type], new_length: usize) -> Result<Type, Error> {
        let (input_array_type, input_array_size) = match inputs.get(Self::ARGUMENT_INDEX_ARRAY) {
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
                    Self::ARGUMENT_COUNT,
                    inputs.len(),
                ))
            }
        };

        match inputs.get(Self::ARGUMENT_INDEX_NEW_LENGTH) {
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
                    Self::ARGUMENT_COUNT,
                    inputs.len(),
                ))
            }
        }

        match inputs.get(Self::ARGUMENT_INDEX_FILL_VALUE) {
            Some(r#type) if r#type.is_scalar() && r#type == &input_array_type => {}
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
                    Self::ARGUMENT_COUNT,
                    inputs.len(),
                ))
            }
        }

        if inputs.get(Self::ARGUMENT_COUNT).is_some() {
            return Err(Error::ArgumentCount(
                self.identifier,
                Self::ARGUMENT_COUNT,
                inputs.len(),
            ));
        }

        if new_length < input_array_size {
            return Err(Error::PadInvalidLength(input_array_size, new_length));
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
