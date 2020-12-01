//!
//! The semantic analyzer standard library `std::collections::MTreeMap::remove` function element.
//!

use std::fmt;

use zinc_lexical::Keyword;
use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::intrinsic::IntrinsicTypeId;

///
/// The semantic analyzer standard library `std::collections::MTreeMap::remove` function element.
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
            library_identifier: LibraryFunctionIdentifier::CollectionsMTreeMapRemove,
            identifier: Self::IDENTIFIER,
        }
    }
}

impl Function {
    /// The function identifier.
    pub const IDENTIFIER: &'static str = "remove";

    /// The position of the `map` argument in the function argument list.
    pub const ARGUMENT_INDEX_SELF: usize = 0;

    /// The position of the `key` argument in the function argument list.
    pub const ARGUMENT_INDEX_KEY: usize = 1;

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

        let (key_type, value_type) = match actual_params.get(Self::ARGUMENT_INDEX_SELF) {
            Some((Type::Structure(structure), _location))
                if structure.type_id == IntrinsicTypeId::StdCollectionsMTreeMap as usize =>
            {
                let key_type = structure
                    .params
                    .as_ref()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .get("K")
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                let value_type = structure
                    .params
                    .as_ref()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
                    .get("V")
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                (key_type, value_type)
            }
            Some((r#type, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: Keyword::SelfLowercase.to_string(),
                    position: Self::ARGUMENT_INDEX_SELF + 1,
                    expected: "std::collections::MTreeMap".to_owned(),
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

        match actual_params.get(Self::ARGUMENT_INDEX_KEY) {
            Some((r#type, _location)) if r#type == key_type => {}
            Some((r#type, location)) => {
                return Err(Error::FunctionArgumentType {
                    location: location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                    function: self.identifier.to_owned(),
                    name: "key".to_owned(),
                    position: Self::ARGUMENT_INDEX_KEY + 1,
                    expected: key_type.to_string(),
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

        if actual_params.len() > Self::ARGUMENT_COUNT {
            return Err(Error::FunctionArgumentCount {
                location,
                function: self.identifier.to_owned(),
                expected: Self::ARGUMENT_COUNT,
                found: actual_params.len(),
                reference: None,
            });
        }

        Ok(Type::tuple(
            None,
            vec![value_type.to_owned(), Type::boolean(None)],
        ))
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "collections::MTreeMap<K, V>::{}(key: K) -> (V, bool)",
            self.identifier
        )
    }
}
