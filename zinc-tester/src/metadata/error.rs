//!
//! The Zinc tester metadata error.
//!

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "parsing: {}", _0)]
    Parsing(serde_json::Error),
}
