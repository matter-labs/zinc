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
    /// The name of the uploaded program.
    pub name: String,
    /// The name of the requested entry within the program.
    pub entry: String,
}

///
/// The program run feature POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The program input JSON data.
    pub input: JsonValue,
}
