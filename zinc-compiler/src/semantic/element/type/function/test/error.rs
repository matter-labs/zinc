//!
//! The semantic analyzer unit test function error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer unit test function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The unit test function cannot be called.
    CallForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function must be only declared at the module root.
    BeyondModuleScope {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot be public.
    PublicForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot be constant.
    ConstantForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot have arguments.
    CannotHaveArguments {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot return a value.
    CannotReturnValue {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
}
