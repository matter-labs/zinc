//!
//! The PostgreSQL program SELECT source models.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL program SELECT source input model.
///
pub struct Input {
    /// The program unique ID.
    pub id: i32,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

///
/// The PostgreSQL program SELECT source output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The program source code.
    pub source: JsonValue,
}
