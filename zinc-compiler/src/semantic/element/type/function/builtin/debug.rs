//!
//! The semantic analyzer `dbg!` built-in function element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_FORMAT_STRING: usize = 0;
    pub const ARGUMENT_INDEX_VALUES: usize = 1;

    pub fn new() -> Self {
        Self {
            location: None,
            identifier: "dbg",
        }
    }

    pub fn call(
        self,
        location: Option<Location>,
        actual_elements: Vec<Element>,
    ) -> Result<(Type, String, Vec<Type>), Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let location = element.location();

            let (r#type, is_constant, string) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::String(inner)) => {
                    (inner.r#type(), true, Some(inner.inner))
                }
                Element::Constant(constant) => (constant.r#type(), true, None),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, is_constant, string, location));
        }

        let format_string = match actual_params.get(Self::ARGUMENT_INDEX_FORMAT_STRING) {
            Some((Type::String(_), true, Some(string), _location)) => string.to_owned(),
            Some((r#type, true, _string, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "format".to_owned(),
                    position: Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    expected: Type::string(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            Some((r#type, false, _string, location)) => {
                return Err(Error::ArgumentConstantness {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "format".to_owned(),
                    position: Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_INDEX_FORMAT_STRING + 1,
                    found: actual_params.len(),
                })
            }
        };

        let arguments_expected_count = format_string.matches("{}").count();
        if arguments_expected_count != actual_params.len() - 1 {
            return Err(Error::BuiltIn(BuiltInFunctionError::DebugArgumentCount {
                location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                expected: arguments_expected_count + 1,
                found: actual_params.len(),
            }));
        }

        let argument_types: Vec<Type> = actual_params
            .into_iter()
            .skip(Self::ARGUMENT_INDEX_VALUES)
            .map(|(r#type, _is_constant, _string, _location)| r#type)
            .collect();

        Ok((Type::unit(None), format_string, argument_types))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}!(format: str, ...)", self.identifier)
    }
}
