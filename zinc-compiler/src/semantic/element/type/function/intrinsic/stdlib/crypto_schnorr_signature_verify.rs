//!
//! The semantic analyzer standard library `std::crypto::schnorr::Signature::verify` function element.
//!

use std::fmt;
use std::ops::Deref;

use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::intrinsic::IntrinsicTypeId;

///
/// The semantic analyzer standard library `std::crypto::schnorr::Signature::verify` function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The unique intrinsic function identifier.
    pub library_identifier: LibraryFunctionIdentifier,
    /// The function identifier.
    pub identifier: &'static str,
    /// The function return type, which is always the same and known.
    pub return_type: Box<Type>,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            location: None,
            library_identifier: LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify,
            identifier: Self::IDENTIFIER,
            return_type: Box::new(Type::boolean(None)),
        }
    }
}

impl Function {
    /// The function identifier.
    pub const IDENTIFIER: &'static str = "verify";

    /// The position of the `signature` argument in the function argument list.
    pub const ARGUMENT_INDEX_SIGNATURE: usize = 0;

    /// The position of the `message` argument in the function argument list.
    pub const ARGUMENT_INDEX_MESSAGE: usize = 1;

    /// The expected number of the function arguments.
    pub const ARGUMENT_COUNT: usize = 2;

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, location: Location, argument_list: ArgumentList) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::FunctionArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_SIGNATURE) {
            Some((Type::Structure(structure), _location))
                if structure.type_id == IntrinsicTypeId::StdCryptoSchnorrSignature as usize => {}
            Some((r#type, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "signature".to_owned(),
                    position: Self::ARGUMENT_INDEX_SIGNATURE + 1,
                    expected: "std::crypto::schnorr::Signature { r: std::crypto::ecc::Point, s: field, pk: std::crypto::ecc::Point }".to_owned(),
                    found: r#type.to_string(),
                })
            },
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

        match actual_params.get(Self::ARGUMENT_INDEX_MESSAGE) {
            Some((Type::Array(array), location)) => match (array.r#type.deref(), array.size) {
                (Type::Boolean(_), size)
                    if size % zinc_const::bitlength::BYTE == 0
                        && size > 0
                        && size <= zinc_const::limit::SCHNORR_MESSAGE_BITS => {}
                (r#type, size) => {
                    return Err(Error::FunctionArgumentType {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        name: "message".to_owned(),
                        position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                        expected: format!(
                            "[bool; N], 0 < N <= {}, N % {} == 0",
                            zinc_const::bitlength::INTEGER_MAX,
                            zinc_const::bitlength::BYTE
                        ),
                        found: format!("array [{}; {}]", r#type, size),
                    });
                }
            },
            Some((r#type, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "message".to_owned(),
                    position: Self::ARGUMENT_INDEX_MESSAGE + 1,
                    expected: format!(
                        "[bool; N], 0 < N <= {}, N % {} == 0",
                        zinc_const::bitlength::INTEGER_MAX,
                        zinc_const::bitlength::BYTE
                    ),
                    found: r#type.to_string(),
                });
            }
            None => {
                return Err(Error::FunctionArgumentCount {
                    location,
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                });
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

        Ok(*self.return_type)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "crypto::schnorr::{}(signature: std::crypto::schnorr::Signature, message: [bool; N]) -> bool", self.identifier)
    }
}
