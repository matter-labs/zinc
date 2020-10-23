//!
//! The Zinc tester application error.
//!

use failure::Fail;

use zinc_build::ValueError as BuildValueError;

///
/// The test application error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The source code could not be compiled succesfully.
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    /// The application input could not be filled from the test metadata.
    #[fail(display = "input value: {}", _0)]
    InputValue(BuildValueError),
    /// The method is missing in the test metadata.
    #[fail(display = "method missing")]
    MethodMissing,
    /// The method could not be found in the test application.
    #[fail(display = "method `{}` not found", _0)]
    MethodNotFound(String),
}
