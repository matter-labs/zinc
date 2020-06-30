//!
//! The compiler resource POST payload.
//!

use serde_derive::Deserialize;

use zinc_compiler::SourceString;

///
/// The compiler resource POST payload.
///
#[derive(Debug, Deserialize)]
pub struct Payload {
    /// The JSON source code tree.
    pub source: SourceString,
}
