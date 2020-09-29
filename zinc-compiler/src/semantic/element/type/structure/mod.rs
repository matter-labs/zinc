//!
//! The semantic analyzer structure type element.
//!

#[cfg(test)]
mod tests;

pub mod error;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;

///
/// Describes a structure type.
///
/// Consists of the local structure `identifier` within its scope, global `type_id`, `fields`,
/// and the implementation `scope`, which contains the reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Structure {
    /// The structure type location in the code.
    pub location: Option<Location>,
    /// The structure type identifier.
    pub identifier: String,
    /// The unique structure type ID.
    pub type_id: usize,
    /// The ordered structure fields array.
    pub fields: Vec<(String, Type)>,
    /// The structure scope, where its methods and associated items are declared.
    pub scope: Rc<RefCell<Scope>>,
}

impl Structure {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Option<Location>,
        identifier: String,
        type_id: usize,
        fields: Vec<(String, Type)>,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = scope.unwrap_or_else(|| Scope::new(identifier.clone(), None).wrap());

        Self {
            location,
            identifier,
            type_id,
            fields,
            scope,
        }
    }
}

impl PartialEq<Self> for Structure {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
