//!
//! The Zinc tester program error.
//!

use failure::Fail;

use zinc_build::ValueError as BuildValueError;

///
/// The test program error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The source code could not be compiled succesfully.
    #[fail(display = "compiler: {}", _0)]
    Compiler(String),
    /// The program input could not be filled from the test metadata.
    #[fail(display = "input value: {}", _0)]
    InputValue(BuildValueError),
    /// The method could not be found in the test program.
    #[fail(display = "method `{}` not found", _0)]
    MethodNotFound(String),
}
