//!
//! The database template INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The database template INSERT input model.
///
pub struct Input {
    /// The template account ID.
    pub account_id: i64,
    /// The template name.
    pub name: String,
    /// The template version.
    pub version: String,
    /// The template source code tree JSON representation.
    pub bytecode: Vec<u8>,
    /// The template contract storage type JSON representation.
    pub storage_type: JsonValue,
    /// The template verifying key as a byte array.
    pub verifying_key: Vec<u8>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        account_id: i64,
        name: String,
        version: String,
        bytecode: Vec<u8>,
        storage_type: JsonValue,
        verifying_key: Vec<u8>,
    ) -> Self {
        Self {
            account_id,

            name,
            version,

            bytecode,
            storage_type,
            verifying_key,
        }
    }
}
