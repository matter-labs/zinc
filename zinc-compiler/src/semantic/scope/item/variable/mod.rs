//!
//! The semantic analyzer scope variable item.
//!

pub mod memory_type;

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;

use self::memory_type::MemoryType;

///
/// The variable item, declared using a `let` statement.
///
#[derive(Debug, Clone)]
pub struct Variable {
    /// The location, where the variable is declared.
    pub location: Location,
    /// The unique item ID, allocated upon declaration.
    pub item_id: usize,
    /// Whether the variable is mutable.
    pub is_mutable: bool,
    /// The variable name.
    pub identifier: String,
    /// The variable type.
    pub r#type: Type,
    /// The memory type, where the variable is declared.
    pub memory_type: MemoryType,
}

impl Variable {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        is_mutable: bool,
        identifier: String,
        r#type: Type,
        memory_type: MemoryType,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("variable {}", identifier));

        Self {
            location,
            item_id,
            is_mutable,
            identifier,
            r#type,
            memory_type,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_mutable {
            write!(f, "mutable {}", self.identifier)
        } else {
            write!(f, "{}", self.identifier)
        }
    }
}
