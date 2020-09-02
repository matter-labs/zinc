//!
//! The Zinc VM bytecode contract program method.
//!

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::data::r#type::Type as BuildType;

///
/// The contract method.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    /// The contract function type unique ID.
    pub type_id: usize,
    /// The contract method address in the bytecode.
    pub address: usize,
    /// Whether the method can mutate the contract storage state.
    pub is_mutable: bool,
    /// The contract method input arguments as a structure.
    pub input: BuildType,
    /// The contract method output type.
    pub output: BuildType,
}

impl Method {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        type_id: usize,
        address: usize,
        is_mutable: bool,
        input: BuildType,
        output: BuildType,
    ) -> Self {
        Self {
            type_id,
            address,
            is_mutable,
            input,
            output,
        }
    }
}
