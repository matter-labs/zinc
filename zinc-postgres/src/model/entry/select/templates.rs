//!
//! The PostgreSQL entry SELECT templates models.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL entry SELECT templates input model.
///
pub struct Input {
    /// The entry unique ID.
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
/// The PostgreSQL entry SELECT templates output model.
///
#[derive(sqlx::FromRow)]
pub struct Output {
    /// The entry input type.
    pub input_type: JsonValue,
    /// The entry output type.
    pub output_type: JsonValue,
    /// The program contract storage structure type.
    pub storage_type: JsonValue,
}
