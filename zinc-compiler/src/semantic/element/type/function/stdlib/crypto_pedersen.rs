//!
//! The semantic analyzer standard library `std::crypto::pedersen` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

#[derive(Debug, Default, Clone)]
pub struct Function {
    identifier: &'static str,
    return_type: Box<Type>,
}

impl Function {
    pub const ARGUMENT_INDEX_PREIMAGE: usize = 0;
    pub const ARGUMENT_COUNT: usize = 1;

    pub fn new() -> Self {
        Self {
            identifier: "pedersen",
            return_type: Box::new(Type::tuple(vec![Type::field(), Type::field()])),
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::CryptoPedersen
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::argument_not_evaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push(r#type);
        }

        match actual_params.get(Self::ARGUMENT_INDEX_PREIMAGE) {
            Some(Type::Array { r#type, size }) => match (r#type.deref(), *size) {
                (Type::Boolean, size)
                    if 0 < size && size <= crate::PEDERSEN_HASH_INPUT_LIMIT_BITS => {}
                (r#type, size) => {
                    return Err(Error::argument_type(
                        self.identifier.to_owned(),
                        "preimage".to_owned(),
                        Self::ARGUMENT_INDEX_PREIMAGE + 1,
                        format!(
                            "[bool; N], 0 < N <= {}",
                            crate::PEDERSEN_HASH_INPUT_LIMIT_BITS
                        ),
                        format!("[{}; {}]", r#type, size),
                    ))
                }
            },
            Some(r#type) => {
                return Err(Error::argument_type(
                    self.identifier.to_owned(),
                    "preimage".to_owned(),
                    Self::ARGUMENT_INDEX_PREIMAGE + 1,
                    format!(
                        "[bool; N], 0 < N <= {}",
                        crate::PEDERSEN_HASH_INPUT_LIMIT_BITS
                    ),
                    r#type.to_string(),
                ))
            }
            None => {
                return Err(Error::argument_count(
                    self.identifier.to_owned(),
                    Self::ARGUMENT_COUNT,
                    actual_params.len(),
                ))
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::argument_count(
                self.identifier.to_owned(),
                Self::ARGUMENT_COUNT,
                actual_params.len(),
            ));
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
