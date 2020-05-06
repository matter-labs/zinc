//!
//! The semantic analyzer standard library `std::crypto::pedersen` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

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
    pub const ARGUMENT_INDEX_PREIMAGE: usize = 0;
    pub const ARGUMENT_COUNT: usize = 1;

    pub fn new(builtin_identifier: BuiltinIdentifier) -> Self {
        Self {
            location: None,
            builtin_identifier,
            identifier: "pedersen",
            return_type: Box::new(Type::tuple(
                Some(Location::default()),
                vec![Type::field(None), Type::field(None)],
            )),
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
                        location: location.unwrap(),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_PREIMAGE) {
            Some((Type::Array(array), location)) => match (array.r#type.deref(), array.size) {
                (Type::Boolean(_), size)
                    if 0 < size && size <= crate::LIMIT_PEDERSEN_HASH_INPUT_BITS => {}
                (r#type, size) => {
                    return Err(Error::ArgumentType {
                        location: location.unwrap(),
                        function: self.identifier.to_owned(),
                        name: "preimage".to_owned(),
                        position: Self::ARGUMENT_INDEX_PREIMAGE + 1,
                        expected: format!(
                            "[bool; N], 0 < N <= {}",
                            crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
                        ),
                        found: format!("[{}; {}]", r#type, size),
                    })
                }
            },
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.unwrap(),
                    function: self.identifier.to_owned(),
                    name: "preimage".to_owned(),
                    position: Self::ARGUMENT_INDEX_PREIMAGE + 1,
                    expected: format!(
                        "[bool; N], 0 < N <= {}",
                        crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
                    ),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.unwrap(),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                })
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.unwrap(),
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
        write!(
            f,
            "fn std::crypto::{}(preimage: [bool: N]) -> {}",
            self.identifier, self.return_type,
        )
    }
}
