//!
//! The semantic analyzer standard library `std::convert::from_bits_field` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::FunctionIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub builtin_identifier: FunctionIdentifier,
    pub identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_BITS: usize = 0;
    pub const ARGUMENT_COUNT: usize = 1;

    pub fn new(builtin_identifier: FunctionIdentifier) -> Self {
        Self {
            location: None,
            builtin_identifier,
            identifier: "from_bits_field",
        }
    }

    pub fn call(
        self,
        location: Option<Location>,
        actual_elements: Vec<Element>,
    ) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        let return_type = match actual_params.get(Self::ARGUMENT_INDEX_BITS) {
            Some((Type::Array(array), location)) => match (array.r#type.deref(), array.size) {
                (Type::Boolean(_), zinc_const::bitlength::FIELD) => Type::field(None),
                (r#type, size) => {
                    return Err(Error::ArgumentType {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: "bits".to_owned(),
                        position: Self::ARGUMENT_INDEX_BITS + 1,
                        expected: format!("[bool; {}]", zinc_const::bitlength::FIELD),
                        found: format!("array [{}; {}]", r#type, size),
                    })
                }
            },
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "bits".to_owned(),
                    position: Self::ARGUMENT_INDEX_BITS + 1,
                    expected: format!("[bool; {}]", zinc_const::bitlength::FIELD),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                })
            }
        };

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
            });
        }

        Ok(return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "convert::{}(bits: [bool; {}]) -> field",
            self.identifier,
            zinc_const::bitlength::FIELD,
        )
    }
}
