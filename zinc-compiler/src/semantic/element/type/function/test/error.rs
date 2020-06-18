//!
//! The semantic analyzer test function error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    CallForbidden {
        location: Location,
        function: String,
    },
    BeyondModuleScope {
        location: Location,
        function: String,
    },
    PublicForbidden {
        location: Location,
        function: String,
    },
    ConstantForbidden {
        location: Location,
        function: String,
    },
    CannotHaveArguments {
        location: Location,
        function: String,
    },
    CannotReturnValue {
        location: Location,
        function: String,
    },
}
