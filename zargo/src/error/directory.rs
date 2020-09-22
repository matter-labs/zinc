//!
//! The generic directory error.
//!

use std::io;

use failure::Fail;

///
/// The generic directory error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The directory creating error.
    #[fail(display = "creating: {}", _0)]
    Creating(io::Error),
    /// The directory removing error.
    #[fail(display = "removing: {}", _0)]
    Removing(io::Error),
}
