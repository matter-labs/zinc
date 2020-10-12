//!
//! The semantic analyzer enumeration type element error.
//!

use num::BigInt;

use zinc_lexical::Location;

///
/// The semantic analyzer enumeration type element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A variant with the same value occurs for the second time.
    DuplicateVariantValue {
        /// The duplicate variant location.
        location: Location,
        /// The enumeration type name.
        type_identifier: String,
        /// The duplicate variant name.
        variant_name: String,
        /// The duplicate variant value.
        variant_value: BigInt,
    },
}
