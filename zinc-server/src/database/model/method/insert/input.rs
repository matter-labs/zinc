//!
//! The database template method INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The database template method INSERT input model.
///
pub struct Input {
    /// The template account ID referencing `templates.account_id`.
    pub template_id: i64,
    /// The method name.
    pub name: String,
    /// If the method can change the contract storage state.
    pub is_mutable: bool,
    /// The method input JSON type.
    pub input_type: JsonValue,
    /// The method output JSON type.
    pub output_type: JsonValue,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        template_id: i64,
        name: String,
        is_mutable: bool,
        input_type: JsonValue,
        output_type: JsonValue,
    ) -> Self {
        Self {
            template_id,
            name,
            is_mutable,
            input_type,
            output_type,
        }
    }
}
