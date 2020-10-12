//!
//! The semantic analyzer expression error.
//!

use crate::semantic::analyzer::expression::conditional::error::Error as ConditionalExpressionError;
use crate::semantic::analyzer::expression::r#match::error::Error as MatchExpressionError;
use zinc_lexical::Location;

///
/// The semantic analyzer expression error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A non-constant element is found in a constant context.
    NonConstantElement {
        /// The error location data.
        location: Location,
        /// The strigified invalid element.
        found: String,
    },

    /// The `match` expression error. See the inner element description.
    Match(MatchExpressionError),
    /// The conditional expression error. See the inner element description.
    Conditional(ConditionalExpressionError),
}
