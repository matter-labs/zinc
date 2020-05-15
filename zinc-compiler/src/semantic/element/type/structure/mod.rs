//!
//! The semantic analyzer structure type element.
//!

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
    pub location: Option<Location>,
    pub identifier: String,
    pub type_id: usize,
    pub fields: Vec<(String, Type)>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Structure {
    pub fn new(
        location: Option<Location>,
        identifier: String,
        type_id: usize,
        fields: Vec<(String, Type)>,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = scope.unwrap_or_else(|| Scope::new(None).wrap());

        let structure = Self {
            location,
            identifier,
            type_id,
            fields,
            scope: scope.clone(),
        };

        scope
            .borrow_mut()
            .define_self(Type::Structure(structure.clone()));

        structure
    }
}

impl PartialEq<Self> for Structure {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
