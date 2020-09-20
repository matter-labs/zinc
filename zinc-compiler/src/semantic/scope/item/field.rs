//!
//! The semantic analyzer scope contract field item.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;

///
/// The contract field item.
///
#[derive(Debug, Clone)]
pub struct Field {
    /// The location, where the variable is declared.
    pub location: Location,
    /// The unique item ID, allocated upon declaration.
    pub item_id: usize,
    /// The variable name.
    pub identifier: String,
    /// The variable type.
    pub r#type: Type,
    /// The index of the field in the contract storage.
    pub index: usize,
    /// Whether the field is public, that is, queryable as a part of the storage state.
    pub is_public: bool,
    /// Whether the field is external, that is, can be set only from outside the contract.
    pub is_external: bool,
}

impl Field {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        identifier: String,
        r#type: Type,
        index: usize,
        is_public: bool,
        is_external: bool,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("field {}", identifier));

        Self {
            location,
            item_id,
            identifier,
            r#type,
            index,
            is_public,
            is_external,
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
