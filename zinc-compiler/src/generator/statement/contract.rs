//!
//! The generator `contract` statement.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Location;

use crate::generator::r#type::contract_field::ContractField as ContractFieldType;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::r#type::contract::field::Field as SemanticContractFieldType;

///
/// The generator `contract` statement.
///
#[derive(Debug, Clone)]
pub struct Statement {
    /// The statement location in the source code.
    pub location: Location,
    /// The `project` section of the contract project manifest.
    pub project: zinc_project::ManifestProject,
    /// The contract storage fields ordered array.
    pub fields: Vec<ContractFieldType>,
    /// Whether the contract is declared within a dependency project.
    pub is_in_dependency: bool,
}

impl Statement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        location: Location,
        project: zinc_project::ManifestProject,
        fields: Vec<SemanticContractFieldType>,
        is_in_dependency: bool,
    ) -> Self {
        Self {
            location,
            project,
            fields: fields
                .into_iter()
                .filter_map(|field| ContractFieldType::try_from_semantic(&field))
                .collect(),
            is_in_dependency,
        }
    }
}

impl IBytecodeWritable for Statement {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        if !self.is_in_dependency {
            state.borrow_mut().set_contract_storage(self.fields);
        }
    }
}
