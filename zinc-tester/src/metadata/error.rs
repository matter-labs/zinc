//!
//! The Zinc tester metadata error.
//!

use failure::Fail;

///
/// The metadata error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The metadata cannot be parsed successfully.
    #[fail(display = "parsing: {}", _0)]
    Parsing(serde_json::Error),
}
