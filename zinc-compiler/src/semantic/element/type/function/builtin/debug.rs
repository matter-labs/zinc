//!
//! The semantic analyzer `dbg!` built-in function type element.
//!

use std::fmt;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::builtin::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    pub fn new() -> Self {
        Self { identifier: "dbg" }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn validate(&self, inputs: &[Element]) -> Result<(Type, String, Vec<Type>), Error> {
        let mut argument_iter = 0usize..;

        let next_argument_index = argument_iter.next().unwrap();
        let format_string = match inputs.get(next_argument_index) {
            Some(Element::Constant(Constant::String(string))) => string.to_owned(),
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
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    next_argument_index + 1,
                    inputs.len(),
                ))
            }
        };

        let arguments_expected_count = format_string.matches("{}").count();
        if arguments_expected_count != inputs.len() - 1 {
            return Err(Error::ArgumentCount(
                self.identifier,
                arguments_expected_count + 1,
                inputs.len(),
            ));
        }

        let next_argument_index = argument_iter.next().unwrap();
        let argument_types: Vec<Type> = inputs
            .iter()
            .skip(next_argument_index)
            .filter_map(|argument| match argument {
                Element::Constant(constant) => Some(constant.r#type()),
                Element::Value(value) => Some(value.r#type()),
                _ => None,
            })
            .collect();

        Ok((Type::unit(), format_string, argument_types))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(format: &str, args: ..)", self.identifier)
    }
}
