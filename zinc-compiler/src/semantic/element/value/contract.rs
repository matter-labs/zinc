//!
//! The semantic analyzer contract value element.
//!

use std::fmt;

use crate::semantic::element::r#type::contract::Contract as ContractType;
use crate::semantic::element::r#type::Type;

///
/// Contracts are collections of named elements of different types.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Contract {
    r#type: ContractType,
    field_index: usize,
}

impl Contract {
    pub fn new(r#type: ContractType) -> Self {
        Self {
            r#type,
            field_index: 0,
        }
    }

    pub fn r#type(&self) -> Type {
        Type::Contract(self.r#type.to_owned())
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type.unique_id == other.r#type.unique_id
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<contract> '{}' with fields {}",
            self.r#type.identifier,
            self.r#type
                .fields
                .iter()
                .map(|(name, r#type)| format!("'{}' of type '{}'", name, r#type))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
