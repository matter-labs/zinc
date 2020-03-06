//!
//! The semantic analyzer `assert!` built-in function element.
//!

use std::fmt;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_CONDITION: usize = 0;
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;
    pub const ARGUMENT_COUNT_MANDATORY: usize = 1;
    pub const ARGUMENT_COUNT_OPTIONAL: usize = 2;

    pub fn new() -> Self {
        Self {
            identifier: "assert",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<(Type, Option<String>), Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let (r#type, is_constant, string) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::String(string)) => (Type::string(), true, Some(string)),
                Element::Constant(constant) => (constant.r#type(), true, None),
                element => {
                    return Err(Error::argument_not_evaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push((r#type, is_constant, string));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_CONDITION) {
            Some((Type::Boolean, _is_constant, _string)) => {}
            Some((r#type, _is_constant, _string)) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "condition".to_owned(),
                    Self::ARGUMENT_INDEX_CONDITION + 1,
                    Type::boolean().to_string(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT_MANDATORY,
                    actual_params.len(),
                ))
            }
        }

        let string = match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some((Type::String, true, string)) => string.to_owned(),
            Some((r#type, true, _string)) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "message".to_owned(),
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    Type::string().to_string(),
                    r#type.to_string(),
                ))
            }
            Some((r#type, false, _string)) => {
                return Err(Error::argument_constantness(
                    self.identifier.to_owned(),
                    "message".to_owned(),
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    r#type.to_string(),
                ))
            }
            None => None,
        };

        if actual_params.len() > Self::ARGUMENT_COUNT_OPTIONAL {
            return Err(Error::argument_count(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT_OPTIONAL,
                actual_params.len(),
            ));
        }

        Ok((Type::unit(), string))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(condition: bool, [message: str])", self.identifier)
    }
}
