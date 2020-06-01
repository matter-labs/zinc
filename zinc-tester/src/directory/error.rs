//!
//! The Zinc tester directory error.
//!

use std::ffi::OsString;
use std::fs::FileType;
use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    #[fail(display = "file entry getting: {}", _0)]
    GettingFileEntry(io::Error),
    #[fail(display = "file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),
    #[fail(display = "invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),
    #[fail(display = "file {:?} extension getting", _0)]
    GettingFileExtension(OsString),
    #[fail(display = "invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(OsString, OsString),
}
