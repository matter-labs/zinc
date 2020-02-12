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
    const ARGUMENT_INDEX_FORMAT_STRING: usize = 0;
    const ARGUMENT_INDEX_VALUES: usize = 1;

    pub fn new() -> Self {
        Self { identifier: "dbg" }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn validate(&self, inputs: &[Element]) -> Result<(Type, String, Vec<Type>), Error> {
        let format_string = match inputs.get(Self::ARGUMENT_INDEX_FORMAT_STRING) {
            Some(Element::Constant(Constant::String(string))) => string.to_owned(),
            Some(Element::Value(value)) => {
                return Err(Error::ArgumentConstantness(
                    self.identifier,
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    value.to_string(),
                ))
            }
            Some(element) => {
                return Err(Error::ArgumentNotEvaluable(
                    self.identifier,
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    element.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier,
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
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

        let argument_types: Vec<Type> = inputs
            .iter()
            .skip(Self::ARGUMENT_INDEX_VALUES)
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
