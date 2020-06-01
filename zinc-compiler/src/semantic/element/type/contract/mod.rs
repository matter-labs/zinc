//!
//! The semantic analyzer contract type element.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

///
/// Describes a contract type.
///
/// Consists of the local contract `identifier` within its scope, global `type_id`,
/// and the implementation `scope`, which contains the reference to its parent scope.
///
#[derive(Debug, Clone)]
pub struct Contract {
    pub location: Option<Location>,
    pub identifier: String,
    pub type_id: usize,
    pub scope: Rc<RefCell<Scope>>,
}

impl Contract {
    pub fn new(
        location: Option<Location>,
        identifier: String,
        type_id: usize,
        scope: Option<Rc<RefCell<Scope>>>,
    ) -> Self {
        let scope = scope.unwrap_or_else(|| Scope::new(identifier.clone(), None).wrap());

        let contract = Self {
            location,
            identifier,
            type_id,
            scope: scope.clone(),
        };

        Scope::insert_item(
            scope,
            Keyword::SelfUppercase.to_string(),
            ScopeItem::Type(ScopeTypeItem::new_defined(
                location,
                Type::Contract(contract.clone()),
                true,
                None,
            ))
            .wrap(),
        );

        contract
    }
}

impl PartialEq<Self> for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.type_id == other.type_id
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
