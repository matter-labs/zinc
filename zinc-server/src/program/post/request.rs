//!
//! The program resource POST request.
//!

use serde_derive::Deserialize;

use zinc_compiler::SourceString;

///
/// The program resource POST request.
///
#[derive(Debug, Deserialize)]
pub struct Request {
    /// The name of the uploaded program.
    pub name: String,
    /// The JSON source code tree.
    pub source: SourceString,
}
