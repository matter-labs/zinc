//!
//! The Zinc tester file error.
//!

use std::io;

use failure::Fail;

///
/// The test file error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The file could not be opened.
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    /// The file metadata could not be acquired.
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    /// The file could not be read.
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
}
