//!
//! The database contract SELECT output model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract SELECT output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The contract account ID.
    pub contract_id: i64,
    /// The contract name.
    pub name: String,
    /// The contract version.
    pub version: String,
    /// The contract source code tree JSON representation.
    pub source_code: JsonValue,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_id: i64, name: String, version: String, source_code: JsonValue) -> Self {
        Self {
            contract_id,
            name,
            version,
            source_code,
        }
    }
}
