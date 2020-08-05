//!
//! The PostgreSQL program INSERT input model.
//!

use serde_json::Value as JsonValue;

///
/// The PostgreSQL program INSERT input model.
///
pub struct Input {
    /// The program name.
    pub name: String,
    /// The program version.
    pub version: String,

    /// The program source code tree JSON representation.
    pub source: JsonValue,
    /// The program contract storage type JSON representation.
    pub storage_type: JsonValue,

    /// The program proving key as a byte array.
    pub proving_key: Vec<u8>,
    /// The program verifying key as a byte array.
    pub verifying_key: Vec<u8>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: String,
        version: String,

        source: JsonValue,
        storage_type: JsonValue,

        proving_key: Vec<u8>,
        verifying_key: Vec<u8>,
    ) -> Self {
        Self {
            name,
            version,

            source,
            storage_type,

            proving_key,
            verifying_key,
        }
    }
}
