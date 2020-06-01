//!
//! The Zinc tester file error.
//!

use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "opening: {}", _0)]
    Opening(io::Error),
    #[fail(display = "metadata: {}", _0)]
    Metadata(io::Error),
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
}
