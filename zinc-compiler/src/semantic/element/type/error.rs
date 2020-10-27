//!
//! The semantic analyzer type error.
//!

use crate::semantic::element::r#type::contract::error::Error as ContractTypeError;
use crate::semantic::element::r#type::enumeration::error::Error as EnumerationTypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;
use zinc_lexical::Location;

///
/// The semantic analyzer type error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The type must be explicitly specified for this binding.
    TypeRequired {
        /// The binding location.
        location: Location,
        /// The binding identifier.
        identifier: String,
    },
    /// The path expression must point to a type, but points to another kind of item.
    AliasDoesNotPointToType {
        /// The path expression location.
        location: Location,
        /// The stringified item, found instead of a type.
        found: String,
    },
    /// The generic items are not supported by the type.
    UnexpectedGenerics {
        /// The path expression location.
        location: Location,
        /// The type which does not expect generics.
        r#type: String,
    },
    /// Some built-in types cannot be instantiated.
    InstantiationForbidden {
        /// The error location.
        location: Location,
        /// The found type.
        found: String,
    },

    /// The function type error. See the inner element description.
    Function(FunctionError),
    /// The structure type error. See the inner element description.
    Structure(StructureTypeError),
    /// The enumeration type error. See the inner element description.
    Enumeration(EnumerationTypeError),
    /// The contract type error. See the inner element description.
    Contract(ContractTypeError),
}
