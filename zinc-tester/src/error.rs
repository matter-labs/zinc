//!
//! The Zinc tester error.
//!

use std::ffi::OsString;

use failure::Fail;

use crate::data::Error as TestDataError;
use crate::directory::Error as TestDirectoryError;
use crate::file::Error as TestFileError;
use crate::program::Error as ProgramDataError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "test directory {}", _0)]
    TestDirectory(TestDirectoryError),
    #[fail(display = "test file {}", _0)]
    TestFile(TestFileError),
    #[fail(display = "test {:?} data {}", _0, _1)]
    TestData(OsString, TestDataError),
    #[fail(display = "program data {}", _0)]
    ProgramData(ProgramDataError),
}
