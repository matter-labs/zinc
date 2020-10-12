//!
//! The semantic analyzer constant tuple element error.
//!

use zinc_lexical::Location;

///
/// The semantic analyzer constant tuple element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The tuple index cannot be greater or equal to the tuple elements count.
    FieldOutOrRange {
        /// The error location data.
        location: Location,
        /// The stringified field type.
        type_identifier: String,
        /// The index that is out of range.
        field_index: usize,
    },
}
