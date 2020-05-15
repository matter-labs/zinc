//!
//! The semantic analyzer contract value element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::contract::Contract as ContractType;
use crate::semantic::element::r#type::Type;

///
/// Contracts are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    pub location: Option<Location>,
    pub r#type: ContractType,
}

impl Contract {
    pub fn new(location: Option<Location>, r#type: ContractType) -> Self {
        Self { location, r#type }
    }

    pub fn r#type(&self) -> Type {
        Type::Contract(self.r#type.to_owned())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type.type_id == other.r#type.type_id
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}'", self.r#type.identifier)
    }
}
