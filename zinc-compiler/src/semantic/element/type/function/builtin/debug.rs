//!
//! The semantic analyzer `dbg!` built-in function element.
//!

use std::fmt;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::error::Error;
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

    pub fn call(self, actual_elements: Vec<Element>) -> Result<(Type, String, Vec<Type>), Error> {
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

        let format_string = match actual_params.get(Self::ARGUMENT_INDEX_FORMAT_STRING) {
            Some((Type::String, true, Some(string))) => string.to_owned(),
            Some((r#type, true, _string)) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "format".to_owned(),
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    Type::string().to_string(),
                    r#type.to_string(),
                ))
            }
            Some((r#type, false, _string)) => {
                return Err(Error::argument_constantness(
                    self.identifier.to_owned(),
                    "format".to_owned(),
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    actual_params.len(),
                ))
            }
        };

        let arguments_expected_count = format_string.matches("{}").count();
        if arguments_expected_count != actual_params.len() - 1 {
            return Err(Error::BuiltIn(BuiltInFunctionError::debug_argument_count(
                arguments_expected_count + 1,
                actual_params.len(),
            )));
        }

        let argument_types: Vec<Type> = actual_params
            .into_iter()
            .skip(Self::ARGUMENT_INDEX_VALUES)
            .map(|(r#type, _is_constant, _string)| r#type)
            .collect();

        Ok((Type::unit(), format_string, argument_types))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(format: str, ...)", self.identifier)
    }
}
