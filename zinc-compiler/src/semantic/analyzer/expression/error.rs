//!
//! The semantic analyzer expression error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::analyzer::expression::conditional::error::Error as ConditionalExpressionError;
use crate::semantic::analyzer::expression::r#match::error::Error as MatchExpressionError;

#[derive(Debug, PartialEq)]
pub enum Error {
    NonConstantElement { location: Location, found: String },

    Match(MatchExpressionError),
    Conditional(ConditionalExpressionError),
}
