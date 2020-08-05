//!
//! The program run feature POST request.
//!

use serde_derive::Deserialize;
use serde_json::Value as JsonValue;

///
/// The program run feature POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The program instance ID.
    pub id: i32,
    /// The program instance entry name.
    pub entry: String,
}

///
/// The program instance run feature POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The program instance entry input JSON data.
    pub input: JsonValue,
}
