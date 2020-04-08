//!
//! The semantic analyzer type error.
//!

use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    AliasDoesNotPointToType { found: String },
    AliasDoesNotPointToStructure { found: String },

    Function(FunctionTypeError),
    Structure(StructureTypeError),
}
