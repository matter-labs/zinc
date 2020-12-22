//!
//! The bytecode contract application method.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::Type;

///
/// The contract method.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// The contract function type unique ID.
    pub type_id: usize,
    /// The contract function name.
    pub name: String,
    /// The contract method address in the bytecode.
    pub address: usize,
    /// Whether the method can mutate the contract storage state.
    pub is_mutable: bool,
    /// The contract method input arguments as a structure.
    pub input: Type,
    /// The contract method output type.
    pub output: Type,
}

impl Method {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        type_id: usize,
        name: String,
        address: usize,
        is_mutable: bool,
        input: Type,
        output: Type,
    ) -> Self {
        Self {
            type_id,
            name,
            address,
            is_mutable,
            input,
            output,
        }
    }
}
