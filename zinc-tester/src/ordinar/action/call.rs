//!
//! The ordinar integration test `call` action.
//!

use std::path::PathBuf;

use serde::Deserialize;

///
/// The ordinar integration test `call` action.
///
#[derive(Debug, Deserialize)]
pub struct Call {
    /// The input JSON template file path.
    pub input_path: PathBuf,
    /// The contract instance to call.
    pub instance: String,
    /// The method name to call.
    pub method: String,
    /// The expected JSON output.
    pub expect: serde_json::Value,
}
