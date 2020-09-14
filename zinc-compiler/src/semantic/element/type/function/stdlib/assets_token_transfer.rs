//!
//! The semantic analyzer standard library `std::assets::Token::transfer` function element.
//!

use std::fmt;

use zinc_build::FunctionIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::function::error::Error;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;

///
/// The semantic analyzer standard library `std::assets::Token::transfer` function element.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The location where the function is called.
    pub location: Option<Location>,
    /// The unique built-in function identifier.
    pub stdlib_identifier: FunctionIdentifier,
    /// The function identifier.
    pub identifier: &'static str,
}

impl Function {
    /// The position of the `from` argument in the function argument list.
    pub const ARGUMENT_INDEX_FROM: usize = 0;

    /// The position of the `to` argument in the function argument list.
    pub const ARGUMENT_INDEX_TO: usize = 1;

    /// The position of the `token` argument in the function argument list.
    pub const ARGUMENT_INDEX_TOKEN_ID: usize = 2;

    /// The position of the `amount` argument in the function argument list.
    pub const ARGUMENT_INDEX_AMOUNT: usize = 3;

    /// The position of the `fee` argument in the function argument list.
    pub const ARGUMENT_INDEX_FEE: usize = 4;

    /// The expected number of the function arguments.
    pub const ARGUMENT_COUNT: usize = 5;

    ///
    /// A shortcut constructor.
    ///
    pub fn new(stdlib_identifier: FunctionIdentifier) -> Self {
        Self {
            location: None,
            stdlib_identifier,
            identifier: "transfer",
        }
    }

    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(
        self,
        location: Option<Location>,
        argument_list: ArgumentList,
    ) -> Result<Type, Error> {
        let mut actual_params = Vec::with_capacity(argument_list.arguments.len());
        for (index, element) in argument_list.arguments.into_iter().enumerate() {
            let location = element.location();

            let r#type = match element {
                Element::Value(value) => value.r#type(),
                Element::Constant(constant) => constant.r#type(),
                element => {
                    return Err(Error::ArgumentNotEvaluable {
                        location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                        function: self.identifier.to_owned(),
                        position: index + 1,
                        found: element.to_string(),
                    })
                }
            };

            actual_params.push((r#type, location));
        }

        match actual_params.get(Self::ARGUMENT_INDEX_FROM) {
            Some((
                Type::IntegerUnsigned {
                    bitlength: zinc_const::bitlength::ETH_ADDRESS,
                    ..
                },
                _location,
            )) => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "from".to_owned(),
                    position: Self::ARGUMENT_INDEX_FROM + 1,
                    expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS)
                        .to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_TO) {
            Some((
                Type::IntegerUnsigned {
                    bitlength: zinc_const::bitlength::ETH_ADDRESS,
                    ..
                },
                _location,
            )) => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "to".to_owned(),
                    position: Self::ARGUMENT_INDEX_TO + 1,
                    expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS)
                        .to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_TOKEN_ID) {
            Some((r#type, _location)) if r#type.is_integer_unsigned() => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "token_id".to_owned(),
                    position: Self::ARGUMENT_INDEX_TOKEN_ID + 1,
                    expected: "{unsigned integer}".to_owned(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_AMOUNT) {
            Some((r#type, _location)) if r#type.is_integer_unsigned() => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "amount".to_owned(),
                    position: Self::ARGUMENT_INDEX_AMOUNT + 1,
                    expected: Type::integer_unsigned(None, zinc_const::bitlength::INTEGER_MAX)
                        .to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        match actual_params.get(Self::ARGUMENT_INDEX_FEE) {
            Some((r#type, _location)) if r#type.is_integer_unsigned() => {}
            Some((r#type, location)) => {
                return Err(Error::ArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "fee".to_owned(),
                    position: Self::ARGUMENT_INDEX_FEE + 1,
                    expected: Type::integer_unsigned(None, zinc_const::bitlength::INTEGER_MAX)
                        .to_string(),
                    found: r#type.to_string(),
                })
            }
            None => {
                return Err(Error::ArgumentCount {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    expected: Self::ARGUMENT_COUNT,
                    found: actual_params.len(),
                    reference: None,
                })
            }
        }

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::ArgumentCount {
                location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
                reference: None,
            });
        }

        Ok(Type::unit(self.location))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "assets::Token::{}(from: u160, to: u160, token_id: u{{N}}, amount: u248, fee: u248)",
            self.identifier
        )
    }
}
