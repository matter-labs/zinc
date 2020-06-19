//!
//! The Zinc constants.
//!

use std::convert::TryFrom;
use std::process::ExitStatus;

pub const BASE_BINARY: usize = 2;
pub const BASE_OCTAL: usize = 8;
pub const BASE_DECIMAL: usize = 10;
pub const BASE_HEXADECIMAL: usize = 16;

pub const BITLENGTH_BOOLEAN: usize = 1;
pub const BITLENGTH_BYTE: usize = 8;
pub const BITLENGTH_INDEX: usize = 64;
pub const BITLENGTH_INTEGER_MAX: usize = 248;
pub const BITLENGTH_FIELD: usize = 254;
pub const BITLENGTH_FIELD_PADDED: usize = 256;

pub const BITLENGTH_SHA256_HASH: usize = 256;
pub const BITLENGTH_SHA256_HASH_TRUNCATED: usize = 248;

///
/// The `1` code is reserved for internal errors.
///
pub enum UnitTestExitCode {
    Passed = 0,
    Failed = 2,
    Ignored = 3,
}

impl TryFrom<ExitStatus> for UnitTestExitCode {
    type Error = Option<i32>;

    fn try_from(status: ExitStatus) -> Result<Self, Self::Error> {
        Ok(match status.code() {
            Some(0) => Self::Passed,
            Some(2) => Self::Failed,
            Some(3) => Self::Ignored,

            code => return Err(code),
        })
    }
}
