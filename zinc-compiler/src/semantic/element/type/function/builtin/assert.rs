//!
//! The semantic analyzer `assert!` built-in function type element.
//!

use std::fmt;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::builtin::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    const ARGUMENT_INDEX_CONDITION: usize = 0;
    const ARGUMENT_INDEX_MESSAGE: usize = 1;
    const ARGUMENT_COUNT: usize = 2;

    pub fn new() -> Self {
        Self {
            identifier: "assert",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn validate(&self, inputs: &[Element]) -> Result<(Type, Option<String>), Error> {
        match inputs.get(Self::ARGUMENT_INDEX_CONDITION) {
            Some(Element::Constant(Constant::Boolean(_))) => {}
            Some(Element::Value(Value::Boolean)) => {}
            Some(Element::Constant(constant)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::boolean().to_string(),
                    Self::ARGUMENT_INDEX_CONDITION + 1,
                    constant.r#type().to_string(),
                ))
            }
            Some(Element::Value(value)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::boolean().to_string(),
                    Self::ARGUMENT_INDEX_CONDITION + 1,
                    value.r#type().to_string(),
                ))
            }
            Some(element) => {
                return Err(Error::ArgumentNotEvaluable(
                    self.identifier,
                    Self::ARGUMENT_INDEX_CONDITION + 1,
                    element.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    Self::ARGUMENT_INDEX_CONDITION + 1,
                    inputs.len(),
                ))
            }
        }

        let string = match inputs.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some(Element::Constant(Constant::String(string))) => Some(string.to_owned()),
            Some(Element::Constant(constant)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::string().to_string(),
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    constant.r#type().to_string(),
                ))
            }
            Some(Element::Value(value)) => {
                return Err(Error::ArgumentConstantness(
                    self.identifier,
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    value.to_string(),
                ))
            }
            Some(element) => {
                return Err(Error::ArgumentNotEvaluable(
                    self.identifier,
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    element.to_string(),
                ))
            }
            None => None,
        };

        if inputs.get(Self::ARGUMENT_COUNT).is_some() {
            return Err(Error::ArgumentCount(
                self.identifier,
                Self::ARGUMENT_COUNT,
                inputs.len(),
            ));
        }

        Ok((Type::unit(), string))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(condition: bool, message: &str)", self.identifier)
    }
}
