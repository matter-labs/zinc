//!
//! The generator `contract` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::r#type::contract_field::ContractField as ContractFieldType;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::r#type::contract::field::Field as SemanticContractFieldType;
use zinc_lexical::Location;

///
/// The Zinc VM storage memory allocating statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The contract storage fields ordered array.
    pub fields: Vec<ContractFieldType>,
    /// Whether the contract is declared within a dependency project.
    pub is_dependency: bool,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        fields: Vec<SemanticContractFieldType>,
        is_dependency: bool,
    ) -> Self {
        Self {
            location,
            fields: fields
                .into_iter()
                .filter_map(|field| ContractFieldType::try_from_semantic(&field))
                .collect(),
            is_dependency,
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_all(self, state: Rc<RefCell<State>>) {
        if !self.is_dependency {
            state.borrow_mut().set_contract_storage(self.fields);
        }
    }
}
