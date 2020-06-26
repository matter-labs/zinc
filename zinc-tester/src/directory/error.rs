//!
//! The Zinc tester directory error.
//!

use std::ffi::OsString;
use std::fs::FileType;
use std::io;

use failure::Fail;

///
/// The test directory error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The directory could not be read.
    #[fail(display = "reading: {}", _0)]
    Reading(io::Error),
    /// The directory file entry could not be acquired.
    #[fail(display = "file entry getting: {}", _0)]
    GettingFileEntry(io::Error),
    /// The directory file type could not be acquired.
    #[fail(display = "file {:?} type getting: {}", _0, _1)]
    GettingFileType(OsString, io::Error),
    /// The directory file type is invalid, that is, the file is neither a data file nor directory.
    #[fail(display = "invalid file {:?} type: {:?}", _0, _1)]
    InvalidFileType(OsString, FileType),
    /// The directory file extension could not be acquired.
    #[fail(display = "file {:?} extension getting", _0)]
    GettingFileExtension(OsString),
    /// The directory file extension is invalid.
    #[fail(display = "invalid file {:?} extension: {:?}", _0, _1)]
    InvalidFileExtension(OsString, OsString),
}
