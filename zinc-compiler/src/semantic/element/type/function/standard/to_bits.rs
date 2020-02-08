//!
//! The semantic analyzer standard library `to_bits` function type element.
//!

use std::fmt;

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
            identifier: "to_bits",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::ToBits
    }

    pub fn arguments_count(&self) -> usize {
        1
    }

    pub fn validate(&self, inputs: &[Type]) -> Result<Type, Error> {
        let result = match inputs.get(0) {
            Some(Type::Boolean) => Ok(Type::array(Type::boolean(), crate::BITLENGTH_BOOLEAN)),
            Some(Type::IntegerUnsigned { bitlength }) => {
                Ok(Type::array(Type::boolean(), *bitlength))
            }
            Some(Type::IntegerSigned { bitlength }) => Ok(Type::array(Type::boolean(), *bitlength)),
            Some(Type::Field) => Ok(Type::array(Type::boolean(), crate::BITLENGTH_FIELD)),
            Some(r#type) => Err(Error::ArgumentType(
                self.identifier,
                "integer".to_owned(),
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
        write!(f, "fn std::{}(value: field) -> [bool: N]", self.identifier,)
    }
}
