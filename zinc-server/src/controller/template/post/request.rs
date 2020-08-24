//!
//! The template resource POST request.
//!

use serde_derive::Deserialize;

use zinc_compiler::SourceString;

///
/// The template resource POST request query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The account ID of the template.
    pub account_id: i64,
    /// The name of the uploaded template.
    pub name: String,
    /// The version of the uploaded template.
    pub version: String,
}

///
/// The template resource POST request body.
///
#[derive(Debug, Deserialize)]
pub struct Body {
    /// The JSON source code tree.
    pub source: SourceString,
}
