//!
//! The semantic analyzer standard library `from_bits_field` function type element.
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

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, Error> {
        let result = match inputs.get(0) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, crate::BITLENGTH_FIELD) => Ok(Type::field()),
                (r#type, size) => Err(Error::ArgumentType(
                    self.identifier,
                    format!("[bool; {}]", crate::BITLENGTH_FIELD),
                    format!("[{}; {}]", r#type, size),
                )),
            },
            Some(r#type) => Err(Error::ArgumentType(
                self.identifier,
                format!("[bool; {}]", crate::BITLENGTH_FIELD),
                r#type.to_string(),
            )),
            None => Err(Error::ArgumentCount(
                self.identifier,
                self.arguments_count(),
                inputs.len(),
            )),
        };

        if inputs.get(1).is_some() {
            return Err(Error::ArgumentCount(
                self.identifier,
                self.arguments_count(),
                inputs.len(),
            ));
        }

        result
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::{}(bits: [bool; {}]) -> field",
            self.identifier,
            crate::BITLENGTH_FIELD,
        )
    }
}
