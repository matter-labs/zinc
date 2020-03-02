//!
//! The semantic analyzer function error.
//!

use crate::lexical::Location;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArgumentCount(String, usize, usize),
    ArgumentType(String, String, usize, String, String),
    ArgumentConstantness(String, usize, String, String),
    ArgumentNotEvaluable(String, usize, String),
    ReturnType(String, String, String, Location),
    NonCallable(String),

    BuiltIn(BuiltInFunctionError),
    StandardLibrary(StandardLibraryFunctionError),
}
