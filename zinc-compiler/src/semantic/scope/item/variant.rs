//!
//! The semantic analyzer scope enumeration variant item.
//!

use std::fmt;

use crate::semantic::element::constant::Constant as ConstantElement;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use zinc_lexical::Location;

///
/// The enumeration variant item, declared using an `enum` statement.
///
#[derive(Debug, Clone)]
pub struct Variant {
    /// The location, where the variant is declared.
    pub location: Location,
    /// The unique variant ID, allocated upon declaration.
    pub item_id: usize,
    /// The semantic constant element.
    pub constant: ConstantElement,
    /// Whether the variant is a part of its enumeration type, and not imported outwards with a `use` statement.
    pub is_associated: bool,
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, identifier: String, constant: ConstantElement) -> Self {
        let item_id = ITEM_INDEX.next(format!("variant {}", identifier));

        Self {
            location,
            item_id,
            constant,
            is_associated: true,
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.constant)
    }
}
