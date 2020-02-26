//!
//! The semantic analyzer standard library `from_bits_field` function type element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    const ARGUMENT_INDEX_BITS: usize = 0;
    const ARGUMENT_COUNT: usize = 1;

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

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push(r#type);
        }

        let return_type = match actual_params.get(Self::ARGUMENT_INDEX_BITS) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, crate::BITLENGTH_FIELD) => Type::field(),
                (r#type, size) => {
                    return Err(Error::ArgumentType(
                        self.identifier.to_owned(),
                        format!("[bool; {}]", crate::BITLENGTH_FIELD),
                        Self::ARGUMENT_INDEX_BITS + 1,
                        "bits".to_owned(),
                        format!("[{}; {}]", r#type, size),
                    ))
                }
            },
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    format!("[bool; {}]", crate::BITLENGTH_FIELD),
                    Self::ARGUMENT_INDEX_BITS + 1,
                    "bits".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        };

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
        }

        Ok(return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::convert::{}(bits: [bool; {}]) -> field",
            self.identifier,
            crate::BITLENGTH_FIELD,
        )
    }
}
