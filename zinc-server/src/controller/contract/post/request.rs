//!
//! The contract resource POST request.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

///
/// The contract resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub account_id: i64,
    /// The template ID to create an instance of.
    pub template_id: i64,
}

///
/// The contract resource POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The instance owner ETH address.
    pub eth_address: String,
    /// The constructor input.
    pub input: JsonValue,
}
