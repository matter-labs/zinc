//!
//! The Zinc unit test exit code constants.
//!

use std::convert::TryFrom;
use std::process::ExitStatus;

///
/// The `1` code is reserved for internal errors.
///
pub enum UnitTestExitCode {
    /// The test passed without an error or with an error if it is marked with the `should_panic` attribute.
    Passed = 0,
    /// The test passed with an error or without an error if it is marked with the `should_panic` attribute.
    Failed = 2,
    /// The test is marked with `ignore` attribute.
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
