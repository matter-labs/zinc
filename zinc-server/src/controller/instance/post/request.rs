//!
//! The program resource POST request.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

///
/// The program resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The program ID to create an instance of.
    pub program_id: i32,
}

///
/// The program resource POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The instance owner address.
    pub owner_address: String,
    /// The constructor input.
    pub input: JsonValue,
}
