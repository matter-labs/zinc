//!
//! The semantic analyzer function error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionTypeError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionTypeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArgumentCount {
        location: Location,
        function: String,
        expected: usize,
        found: usize,
    },
    ArgumentType {
        location: Location,
        function: String,
        name: String,
        position: usize,
        expected: String,
        found: String,
    },
    ArgumentConstantness {
        location: Location,
        function: String,
        name: String,
        position: usize,
        found: String,
    },
    ArgumentNotEvaluable {
        location: Location,
        function: String,
        position: usize,
        found: String,
    },
    ReturnType {
        location: Location,
        function: String,
        expected: String,
        found: String,
        reference: Location,
    },
    NonCallable {
        location: Location,
        name: String,
    },
    FunctionMethodSelfNotFirst {
        location: Location,
        function: String,
        position: usize,
        reference: Location,
    },

    BuiltIn(BuiltInFunctionTypeError),
    StandardLibrary(StandardLibraryFunctionTypeError),
}
