//!
//! The generator `contract` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;
use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    pub location: Location,
    pub fields: Vec<(String, Type)>,
}

impl Statement {
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

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        bytecode.borrow_mut().set_contract_storage(self.fields)
    }
}
