//!
//! The semantic analyzer standard library `std::crypto::schnorr::verify` function element.
//!

use std::fmt;

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
    const ARGUMENT_INDEX_R_X: usize = 0;
    const ARGUMENT_INDEX_R_Y: usize = 1;
    const ARGUMENT_INDEX_S: usize = 2;
    const ARGUMENT_INDEX_PK_X: usize = 3;
    const ARGUMENT_INDEX_PK_Y: usize = 4;
    const ARGUMENT_INDEX_MESSAGE: usize = 5;
    const ARGUMENT_COUNT: usize = 6;

    pub fn new() -> Self {
        Self {
            identifier: "verify",
            return_type: Box::new(Type::boolean()),
        }
    }

    pub fn identifier(&self) -> &'static str {
        self.identifier
    }

    pub fn builtin_identifier(&self) -> BuiltinIdentifier {
        BuiltinIdentifier::CryptoSchnorrVerify
    }

    pub fn call(self, actual_elements: Vec<Element>) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(actual_elements.len());
        for (index, element) in actual_elements.into_iter().enumerate() {
            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable(
                        self.identifier.to_owned(),
                        index + 1,
                        element.to_string(),
                    ))
                }
            };
            actual_params.push(r#type);
        }

        match actual_params.get(Self::ARGUMENT_INDEX_R_X) {
            Some(Type::Field) => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    Type::field().to_string(),
                    Self::ARGUMENT_INDEX_R_X + 1,
                    "r_x".to_owned(),
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
        }

        match actual_params.get(Self::ARGUMENT_INDEX_R_Y) {
            Some(Type::Field) => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    Type::field().to_string(),
                    Self::ARGUMENT_INDEX_R_Y + 1,
                    "r_y".to_owned(),
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
        }

        match actual_params.get(Self::ARGUMENT_INDEX_S) {
            Some(Type::Field) => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    Type::field().to_string(),
                    Self::ARGUMENT_INDEX_S + 1,
                    "s".to_owned(),
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
        }

        match actual_params.get(Self::ARGUMENT_INDEX_PK_X) {
            Some(Type::Field) => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    Type::field().to_string(),
                    Self::ARGUMENT_INDEX_PK_X + 1,
                    "pk_x".to_owned(),
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
        }

        match actual_params.get(Self::ARGUMENT_INDEX_PK_Y) {
            Some(Type::Field) => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    Type::field().to_string(),
                    Self::ARGUMENT_INDEX_PK_Y + 1,
                    "pk_y".to_owned(),
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
        }

        match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some(r#type) if r#type.is_bit_array() => {}
            Some(r#type) => {
                return Err(Error::ArgumentType(
                    self.identifier.to_owned(),
                    "[bool; {N}]".to_owned(),
                    Self::ARGUMENT_INDEX_MESSAGE + 1,
                    "message".to_owned(),
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
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount(
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
        write!(f, "fn std::crypto::schnorr::{}(r_x: field, r_y: field, s: field, pk_x: field, pk_y: field, message: field) -> bool", self.identifier)
    }
}
