//!
//! The `contract storage init` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::contract_field::ContractField;
use crate::instructions::Instruction;

///
/// The `contract storage init` instruction.
///
/// Initializes a contract storage represented by `fields` and pushes its address onto the
/// evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageInit {
    /// The `project` section of the contract project manifest.
    pub project: zinc_project::ManifestProject,
    /// The contract storage fields type metadata.
    pub field_types: Vec<ContractField>,
}

impl StorageInit {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(project: zinc_project::ManifestProject, field_types: Vec<ContractField>) -> Self {
        Self {
            project,
            field_types,
        }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for StorageInit {
    fn into(self) -> Instruction {
        Instruction::StorageInit(self)
    }
}

impl fmt::Display for StorageInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "storage_init {} ({}-{})",
            self.field_types
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.project.name,
            self.project.version,
        )
    }
}
