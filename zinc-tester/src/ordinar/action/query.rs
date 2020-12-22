//!
//! The ordinar integration test `query` action.
//!

use std::path::PathBuf;

use serde::Deserialize;

///
/// The ordinar integration test `query` action.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The input JSON template file path.
    pub input_path: PathBuf,
    /// The contract instance to query.
    pub instance: String,
    /// The optional method name to query. If not set, the storage is queried.
    pub method: Option<String>,
    /// The expected JSON output.
    pub expect: serde_json::Value,
}
