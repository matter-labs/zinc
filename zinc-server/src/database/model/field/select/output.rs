//!
//! The database contract storage field SELECT output model.
//!

use serde_json::Value as JsonValue;

///
/// The database contract storage field SELECT output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The field name.
    pub name: String,
    /// The field value in JSON representation.
    pub value: JsonValue,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, value: JsonValue) -> Self {
        Self { name, value }
    }
}
