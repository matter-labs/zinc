//!
//! The generic file error.
//!

use std::fmt;
use std::io;

use failure::Fail;

///
/// The generic file error.
///
#[derive(Debug, Fail)]
pub enum Error<P = String>
where
    P: fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    /// File opening error.
    #[fail(display = "`{}` opening: {}", _0, _1)]
    Opening(String, io::Error),
    /// File metadata getting error.
    #[fail(display = "`{}` metadata: {}", _0, _1)]
    Metadata(String, io::Error),
    /// File reading error.
    #[fail(display = "`{}` reading: {}", _0, _1)]
    Reading(String, io::Error),
    /// File contents parsing error.
    #[fail(display = "`{}` parsing: {}", _0, _1)]
    Parsing(String, P),
    /// File creating error.
    #[fail(display = "`{}` creating: {}", _0, _1)]
    Creating(String, io::Error),
    /// File writing error.
    #[fail(display = "`{}` writing: {}", _0, _1)]
    Writing(String, io::Error),
}
