//!
//! The semantic attribute error.
//!

use zinc_lexical::Location;

///
/// The semantic attribute error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The attribute is unknown. Check the known attribute list for more information.
    Unknown {
        /// The error location data.
        location: Location,
        /// The invalid stringified attribute.
        found: String,
    },
}
