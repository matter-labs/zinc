//!
//! The semantic analyzer intrinsic function error.
//!

use crate::semantic::element::r#type::function::intrinsic::debug::error::Error as DebugFunctionError;
use crate::semantic::element::r#type::function::intrinsic::stdlib::error::Error as StandardLibraryFunctionError;
use zinc_lexical::Location;

///
/// The semantic analyzer intrinsic function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Tried to call a function with the `!` specifier, but the function does not require it.
    Unknown {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// Some intrinsic functions can only be called with the `!` specifier.
    ExclamationMarkMissing {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: &'static str,
    },
    /// The `dbg!(...)` function error.
    Debug(DebugFunctionError),
    /// The standary library function error.
    StandardLibrary(StandardLibraryFunctionError),
}
