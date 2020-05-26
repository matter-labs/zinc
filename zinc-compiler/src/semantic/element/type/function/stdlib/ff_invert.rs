//!
//! The semantic analyzer standard library `std::ff::invert` function element.
//!

use std::fmt;

use zinc_bytecode::BuiltinIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub builtin_identifier: BuiltinIdentifier,
    pub identifier: &'static str,
    pub return_type: Box<Type>,
}

impl Function {
    pub const ARGUMENT_INDEX_VALUE: usize = 0;
    pub const ARGUMENT_COUNT: usize = 1;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            location: None,
            builtin_identifier,
            identifier: "invert",
            return_type: Box::new(Type::field(None)),
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

        match actual_params.get(Self::ARGUMENT_INDEX_VALUE) {
            Some((Type::Field(_), _location)) => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "value".to_owned(),
                    position: Self::ARGUMENT_INDEX_VALUE + 1,
                    expected: Type::field(None).to_string(),
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
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
            });
        }

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ff::{}(value: field) -> field", self.identifier,)
    }
}
