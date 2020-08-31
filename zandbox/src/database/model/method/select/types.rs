//!
//! The database method SELECT templates models.
//!

use serde_json::Value as JsonValue;

///
/// The database method SELECT templates input model.
///
pub struct Input {
    /// The method contract ID.
    pub contract_id: i64,
    /// The method unique name within the template.
    pub name: String,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, name: String) -> Self {
        Self { contract_id, name }
    }
}

///
/// The database method SELECT templates output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The method input type.
    pub input_type: JsonValue,
    /// The method output type.
    pub output_type: JsonValue,
    /// The template contract storage structure type.
    pub storage_type: JsonValue,
}
