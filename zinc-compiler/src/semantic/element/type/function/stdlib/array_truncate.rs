//!
//! The semantic analyzer standard library `std::array::truncate` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::function::stdlib::error::Error as StdlibError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
}

impl Function {
    const ARGUMENT_INDEX_ARRAY: usize = 0;
    const ARGUMENT_INDEX_NEW_LENGTH: usize = 1;
    const ARGUMENT_COUNT: usize = 2;

    pub fn new() -> Self {
        Self {
            identifier: "truncate",
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::ArrayTruncate
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let (r#type, is_constant, number) = match element {
                Element::Value(value) => (value.r#type(), false, None),
                Element::Constant(Constant::Integer(integer)) => (
                    integer.r#type(),
                    true,
                    integer
                        .to_usize()
                        .map(Option::Some)
                        .map_err(|_error| StdlibError::ArrayNewLengthInvalid(integer.to_string()))
                        .map_err(Error::StandardLibrary)?,
                ),
                Element::Constant(constant) => (constant.r#type(), true, None),
                element => {
                    return Err(Error::ArgumentNotEvaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push((r#type, is_constant, number));
        }

        let (input_array_type, input_array_size) =
            match actual_params.get(Self::ARGUMENT_INDEX_ARRAY) {
                Some((Type::Array { r#type, size }, _is_constant, _is_number))
                    if r#type.is_scalar() =>
                {
                    (r#type.deref().to_owned(), *size)
                }
                Some((r#type, _is_constant, _is_number)) => {
                    return Err(Error::ArgumentType(
                        self.identifier.to_owned(),
                        "[{scalar}; {N}]".to_owned(),
                        Self::ARGUMENT_INDEX_ARRAY + 1,
                        "array".to_owned(),
                        r#type.to_string(),
                    ))
                }
                None => {
                    return Err(Error::ArgumentCount(
                        self.identifier.to_owned(),
                        Self::ARGUMENT_COUNT,
                        actual_params.len(),
                    ))
                }
            };

        let new_length = match actual_params.get(Self::ARGUMENT_INDEX_NEW_LENGTH) {
            Some((r#type, true, Some(number))) if r#type.is_scalar_unsigned() => *number,
            Some((r#type, true, _number)) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    "{unsigned integer}".to_owned(),
                    Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    "new_length".to_owned(),
                    r#type.to_string(),
                ))
            }
            Some((r#type, false, _number)) => {
                return Err(Error::ArgumentConstantness(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_INDEX_NEW_LENGTH + 1,
                    "new_length".to_owned(),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::ArgumentCount(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        };

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
        }

        if new_length > input_array_size {
            return Err(Error::StandardLibrary(
                StdlibError::ArrayTruncatingToBiggerSize(input_array_size, new_length),
            ));
        }

        Ok(Type::array(input_array_type, new_length))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn std::array::{}(array: [{{T: scalar}}; {{N}}], new_length: {{M: unsigned integer}}) -> [{{T: scalar}}; new_length]",
            self.identifier,
        )
    }
}
