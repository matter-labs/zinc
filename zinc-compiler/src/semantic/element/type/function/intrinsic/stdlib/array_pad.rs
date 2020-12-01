//!
//! The semantic analyzer standard library `std::array::pad` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

///
/// The semantic analyzer standard library `std::array::pad` function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The unique intrinsic function identifier.
    pub library_identifier: LibraryFunctionIdentifier,
    /// The function identifier.
    pub identifier: &'static str,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            location: None,
            library_identifier: LibraryFunctionIdentifier::ArrayPad,
            identifier: Self::IDENTIFIER,
        }
    }
}

impl Function {
    /// The function identifier.
    pub const IDENTIFIER: &'static str = "pad";

    /// The position of the `array` argument in the function argument list.
    pub const ARGUMENT_INDEX_ARRAY: usize = 0;

    /// The position of the `new_length` argument in the function argument list.
    pub const ARGUMENT_INDEX_NEW_LENGTH: usize = 1;

    /// The position of the `fill_value` argument in the function argument list.
    pub const ARGUMENT_INDEX_FILL_VALUE: usize = 2;

    /// The expected number of the function arguments.
    pub const ARGUMENT_COUNT: usize = 3;

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, location: Location, argument_list: ArgumentList) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let (r#type, is_constant, number) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::Integer(integer)) => {
                    let number = integer.to_usize().map_err(|_error| {
                        Error::FunctionStdlibArrayNewLengthInvalid {
                            location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            value: integer.to_string(),
                        }
                    })?;

                    (integer.r#type(), true, Some(number))
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
                    return Err(Error::FunctionArgumentType {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: "array".to_owned(),
                        position: Self::ARGUMENT_INDEX_ARRAY + 1,
                        expected: "[{scalar}; N]".to_owned(),
                        found: r#type.to_string(),
                    })
                }
                None => {
                    return Err(Error::FunctionArgumentCount {
                        location,
                        function: self.identifier.to_owned(),
                        expected: Self::ARGUMENT_COUNT,
                        found: actual_params.len(),
                        reference: None,
                    })
                }
            };

        let new_length = match actual_params.get(Self::ARGUMENT_INDEX_NEW_LENGTH) {
            Some((r#type, true, Some(number), _location)) if r#type.is_scalar_unsigned() => *number,
            Some((r#type, true, _number, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "new_length".to_owned(),
                    position: Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    expected: "{unsigned integer}".to_owned(),
                    found: r#type.to_string(),
                })
            }
            Some((r#type, false, _number, location)) => {
                return Err(Error::FunctionArgumentConstantness {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "new_length".to_owned(),
                    position: Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        };

        match actual_params.get(Self::ARGUMENT_INDEX_FILL_VALUE) {
            Some((r#type, _is_constant, _number, _location))
                if r#type.is_scalar() && r#type == &input_array_type => {}
            Some((r#type, _is_constant, _number, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "fill_value".to_owned(),
                    position: Self::ARGUMENT_INDEX_FILL_VALUE + 1,
                    expected: input_array_type.to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::FunctionArgumentCount {
                location,
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
                reference: None,
            });
        }

        if new_length < input_array_size {
            return Err(Error::FunctionStdlibArrayPaddingToLesserSize {
                location,
                from: input_array_size,
                to: new_length,
            });
        }

        Ok(Type::array(Some(location), input_array_type, new_length))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "array::{}(array: [T; N], new_length: M, fill_value: T) -> [T; M]",
            self.identifier,
        )
    }
}
