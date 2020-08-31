//!
//! The contract resource POST request.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

use zinc_source::Source;

///
/// The contract resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The contract account ID.
    pub contract_id: i64,
    /// The name of the uploaded contract.
    pub name: String,
    /// The version of the uploaded contract.
    pub version: String,
}

///
/// The contract resource POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The JSON source code tree.
    pub source: Source,
    /// The constructor input.
    pub constructor_input: JsonValue,
}
