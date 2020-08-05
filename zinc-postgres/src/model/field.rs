//!
//! The PostgreSQL program instance contract storage field model.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL program instance contract storage field model.
///
pub struct Field {
    /// The field index in the contract storage.
    pub index: u16,

    /// The instance ID referencing `instances.id`.
    pub program_id: u32,

    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Field {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(index: u16, program_id: u32, value: JsonValue) -> Self {
        Self {
            index,

            program_id,

            value,
        }
    }
}
