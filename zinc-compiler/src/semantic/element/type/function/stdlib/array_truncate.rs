//!
//! The semantic analyzer standard library `std::array::truncate` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::FunctionIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::function::stdlib::error::Error as StdlibError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Option<Location>,
    pub builtin_identifier: FunctionIdentifier,
    pub identifier: &'static str,
}

impl Function {
    pub const ARGUMENT_INDEX_ARRAY: usize = 0;
    pub const ARGUMENT_INDEX_NEW_LENGTH: usize = 1;
    pub const ARGUMENT_COUNT: usize = 2;

    pub fn new(builtin_identifier: FunctionIdentifier) -> Self {
        Self {
            location: None,
            builtin_identifier,
            identifier: "truncate",
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

            let (r#type, is_constant, number) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::Integer(integer)) => {
                    let number = integer
                        .to_usize()
                        .map_err(|_error| StdlibError::ArrayNewLengthInvalid {
                            location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                            value: integer.to_string(),
                        })
                        .map_err(Error::StandardLibrary)?;

                    (integer.r#type(), true, Some(number))
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

            actual_params.push((r#type, is_constant, number, location));
        }

        let (input_array_type, input_array_size) =
            match actual_params.get(Self::ARGUMENT_INDEX_ARRAY) {
                Some((Type::Array(array), _is_constant, _number, _location))
                    if array.r#type.is_scalar() =>
                {
                    (array.r#type.deref().to_owned(), array.size)
                }
                Some((r#type, _is_constant, _number, location)) => {
                    return Err(Error::ArgumentType {
                        location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: "array".to_owned(),
                        position: Self::ARGUMENT_INDEX_ARRAY + 1,
                        expected: "[{scalar}; N]".to_owned(),
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

        let new_length = match actual_params.get(Self::ARGUMENT_INDEX_NEW_LENGTH) {
            Some((r#type, true, Some(number), _location)) if r#type.is_scalar_unsigned() => *number,
            Some((r#type, true, _number, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "new_length".to_owned(),
                    position: Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    expected: "{unsigned integer}".to_owned(),
                    found: r#type.to_string(),
                })
            }
            Some((r#type, false, _number, location)) => {
                return Err(Error::ArgumentConstantness {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "new_length".to_owned(),
                    position: Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
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

        if new_length > input_array_size {
            return Err(Error::StandardLibrary(
                StdlibError::ArrayTruncatingToBiggerSize {
                    location: location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
                    from: input_array_size,
                    to: new_length,
                },
            ));
        }

        Ok(Type::array(location, input_array_type, new_length))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "array::{}(array: [T; N], new_length: M) -> [T; M]",
            self.identifier,
        )
    }
}
