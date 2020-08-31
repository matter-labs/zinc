//!
//! The contract resource PUT query request.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

///
/// The contract resource PUT query request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub contract_id: i64,
    /// The method name to call.
    pub method: String,
}

///
/// The contract resource PUT query request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The method input.
    pub input: JsonValue,
}
