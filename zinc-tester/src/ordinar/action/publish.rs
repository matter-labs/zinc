//!
//! The ordinar integration test `publish` action.
//!

use std::path::PathBuf;

use serde::Deserialize;

///
/// The ordinar integration test `publish` action.
///
#[derive(Debug, Deserialize)]
pub struct Publish {
    /// The published instance name.
    pub instance: String,
    /// The input JSON template file path.
    pub input_path: PathBuf,
    /// The change-pubkey fee token.
    pub change_pubkey_fee_token: String,
}
