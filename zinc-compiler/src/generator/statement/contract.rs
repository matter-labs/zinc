//!
//! The generator `contract` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::r#type::Type;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The contract storage fields ordered array.
    pub fields: Vec<(String, Type)>,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, fields: Vec<(String, SemanticType)>) -> Self {
        Self {
            location,
            fields: fields
                .into_iter()
                .filter_map(|(name, r#type)| match Type::try_from_semantic(&r#type) {
                    Some(r#type) => Some((name, r#type)),
                    None => None,
                })
                .collect(),
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_all(self, state: Rc<RefCell<State>>) {
        state.borrow_mut().set_contract_storage(self.fields)
    }
}
