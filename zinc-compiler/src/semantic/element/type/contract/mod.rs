//!
//! The semantic analyzer contract type element.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;

///
/// Describes a contract type.
///
/// Consists of the local contract `identifier` within its scope, global `unique_id`, `fields`,
/// and the implementation `scope`, which contains the reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Contract {
    pub identifier: String,
    pub unique_id: usize,
    pub fields: Vec<(String, Type)>,
    pub scope: Rc<RefCell<Scope>>,
}

impl Contract {
    pub fn new(
        identifier: String,
        unique_id: usize,
        fields: Vec<(String, Type)>,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = scope.unwrap_or_else(|| Rc::new(RefCell::new(Scope::new(None))));

        let contract = Self {
            identifier,
            unique_id,
            fields,
            scope: scope.clone(),
        };

        scope
            .borrow_mut()
            .declare_self(Type::Contract(contract.clone()));

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
