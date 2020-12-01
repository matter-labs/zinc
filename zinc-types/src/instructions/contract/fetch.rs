//!
//! The `contract storage fetch` instruction.
//!

use std::fmt;

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::contract_field::ContractField;
use crate::instructions::Instruction;

///
/// The `contract storage fetch` instruction.
///
/// Fetches the contract storage represented by `fields` from the contract server and
/// loads it onto the evaluation stack.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageFetch {
    /// The contract storage fields type metadata.
    pub field_types: Vec<ContractField>,
}

impl StorageFetch {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(field_types: Vec<ContractField>) -> Self {
        Self { field_types }
    }

    ///
    /// If the instruction is for the debug mode only.
    ///
    pub fn is_debug(&self) -> bool {
        false
    }
}

impl Into<Instruction> for StorageFetch {
    fn into(self) -> Instruction {
        Instruction::StorageFetch(self)
    }
}

impl fmt::Display for StorageFetch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "storage_fetch {}",
            self.field_types
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
