//!
//! The semantic analyzer `assert!` built-in function element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_CONDITION: usize = 0;
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;
    pub const ARGUMENT_COUNT_MANDATORY: usize = 1;
    pub const ARGUMENT_COUNT_OPTIONAL: usize = 2;

    pub fn new() -> Self {
        Self {
            location: None,
            identifier: "assert",
        }
    }

    pub fn call(
        self,
        location: Option<Location>,
        actual_elements: Vec<Element>,
    ) -> Result<(Type, Option<String>), Error> {
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

        match actual_params.get(Self::ARGUMENT_INDEX_CONDITION) {
            Some((Type::Boolean(_), _is_constant, _string, _location)) => {}
            Some((r#type, _is_constant, _string, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "condition".to_owned(),
                    position: Self::ARGUMENT_INDEX_CONDITION + 1,
                    expected: Type::boolean(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT_MANDATORY,
                    found: actual_params.len(),
                })
            }
        }

        let string = match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some((Type::String(_), true, string, _location)) => string.to_owned(),
            Some((r#type, true, _string, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    expected: Type::string(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            Some((r#type, false, _string, location)) => {
                return Err(Error::ArgumentConstantness {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    found: r#type.to_string(),
                });
            }
            None => None,
        };

        if actual_params.len() > Self::ARGUMENT_COUNT_OPTIONAL {
            return Err(Error::ArgumentCount {
                location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT_OPTIONAL,
                found: actual_params.len(),
            });
        }

        Ok((Type::unit(None), string))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}!(condition: bool, [message: str])", self.identifier)
    }
}
