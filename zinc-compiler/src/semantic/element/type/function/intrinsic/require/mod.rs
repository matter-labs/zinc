//!
//! The semantic analyzer `require` intrinsic function element.
//!

#[cfg(test)]
mod tests;

use std::fmt;

use zinc_lexical::Location;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The semantic analyzer `require` intrinsic function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The function identifier.
    pub identifier: &'static str,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            location: None,
            identifier: Self::IDENTIFIER,
        }
    }
}

impl Function {
    /// The function identifier.
    pub const IDENTIFIER: &'static str = "require";

    /// The position of the `condition` argument in the function argument list.
    pub const ARGUMENT_INDEX_CONDITION: usize = 0;

    /// The position of the optional `message` argument in the function argument list.
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;

    /// The number of arguments, not including the optional ones.
    pub const ARGUMENT_COUNT_MANDATORY: usize = 1;

    /// The number of arguments, including the optional ones.
    pub const ARGUMENT_COUNT_OPTIONAL: usize = 2;

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(
        self,
        location: Location,
        argument_list: ArgumentList,
    ) -> Result<(Type, Option<String>), Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let (r#type, is_constant, string) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::String(inner)) => {
                    (inner.r#type(), true, Some(inner.inner))
                }
                Element::Constant(constant) => (constant.r#type(), true, None),
                element => {
                    return Err(Error::FunctionArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
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
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "condition".to_owned(),
                    position: Self::ARGUMENT_INDEX_CONDITION + 1,
                    expected: Type::boolean(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT_MANDATORY,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        let string = match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some((Type::String(_), true, string, _location)) => string.to_owned(),
            Some((r#type, true, _string, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    expected: Type::string(None).to_string(),
                    found: r#type.to_string(),
                })
            }
            Some((r#type, false, _string, location)) => {
                return Err(Error::FunctionArgumentConstantness {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    found: r#type.to_string(),
                });
            }
            None => None,
        };

        if actual_params.len() > Self::ARGUMENT_COUNT_OPTIONAL {
            return Err(Error::FunctionArgumentCount {
                location,
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT_OPTIONAL,
                found: actual_params.len(),
                reference: None,
            });
        }

        Ok((Type::unit(None), string))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(condition: bool, [message: str])", self.identifier)
    }
}
