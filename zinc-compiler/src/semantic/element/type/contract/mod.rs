//!
//! The semantic analyzer contract type element.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::r#type::index::SOFT as TYPE_INDEX_SOFT;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::Scope;

///
/// Describes a contract type.
///
/// Consists of the local contract `identifier` within its scope, global `unique_id`, `fields`,
/// and the implementation `scope`, which contains the reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Contract {
    pub location: Option<Location>,
    pub identifier: String,
    pub unique_id: usize,
    pub scope: Rc<RefCell<Scope>>,
}

impl Contract {
    pub fn new(
        location: Option<Location>,
        identifier: String,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = scope.unwrap_or_else(|| Rc::new(RefCell::new(Scope::new(None))));

        let unique_id = TYPE_INDEX_SOFT.next(identifier.clone());
        let contract = Self {
            location,
            identifier,
            unique_id,
            scope: scope.clone(),
        };

        scope.borrow_mut().declare_self(ScopeTypeItem::new(
            location,
            Type::Contract(contract.clone()),
        ));

        contract
    }
}

impl PartialEq<Self> for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id == other.unique_id
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "contract {}", self.identifier)
    }
}
