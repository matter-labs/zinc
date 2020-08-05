//!
//! The PostgreSQL program entry INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL program entry INSERT input model.
///
pub struct Input {
    /// The program ID referencing `programs.id`.
    pub program_id: i32,

    /// The entry name.
    pub name: String,
    /// If the entry can change the contract storage.
    pub is_mutable: bool,

    /// The entry input JSON type.
    pub input_type: JsonValue,
    /// The entry output JSON type.
    pub output_type: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        program_id: i32,

        name: String,
        is_mutable: bool,

        input_type: JsonValue,
        output_type: JsonValue,
    ) -> Self {
        Self {
            program_id,

            name,
            is_mutable,

            input_type,
            output_type,
        }
    }
}
