//!
//! The semantic analyzer type error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::contract::error::Error as ContractTypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    AliasDoesNotPointToType { location: Location, found: String },

    Function(FunctionError),
    Structure(StructureTypeError),
    Contract(ContractTypeError),
}
