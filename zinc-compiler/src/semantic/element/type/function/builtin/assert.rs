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
    pub fn new() -> Self {
        Self {
            identifier: "assert",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn validate(&self, inputs: &[Element]) -> Result<(Type, Option<String>), Error> {
        let mut argument_iter = 0usize..;

        let next_argument_index = argument_iter.next().unwrap();
        match inputs.get(next_argument_index) {
            Some(Element::Constant(Constant::Boolean(_))) => {}
            Some(Element::Value(Value::Boolean)) => {}
            Some(Element::Constant(constant)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::boolean().to_string(),
                    next_argument_index + 1,
                    constant.r#type().to_string(),
                ))
            }
            Some(Element::Value(value)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::boolean().to_string(),
                    next_argument_index + 1,
                    value.r#type().to_string(),
                ))
            }
            Some(element) => {
                return Err(Error::ArgumentNotEvaluable(
                    self.identifier,
                    next_argument_index + 1,
                    element.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    next_argument_index + 1,
                    inputs.len(),
                ))
            }
        }

        let next_argument_index = argument_iter.next().unwrap();
        let string = match inputs.get(next_argument_index) {
            Some(Element::Constant(Constant::String(string))) => Some(string.to_owned()),
            Some(Element::Constant(constant)) => {
                return Err(Error::ArgumentType(
                    self.identifier,
                    Type::string().to_string(),
                    next_argument_index + 1,
                    constant.r#type().to_string(),
                ))
            }
            Some(Element::Value(value)) => {
                return Err(Error::ArgumentConstantness(
                    self.identifier,
                    next_argument_index + 1,
                    value.to_string(),
                ))
            }
            Some(element) => {
                return Err(Error::ArgumentNotEvaluable(
                    self.identifier,
                    next_argument_index + 1,
                    element.to_string(),
                ))
            }
            None => None,
        };

        Ok((Type::unit(), string))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(condition: bool, message: &str)", self.identifier)
    }
}
