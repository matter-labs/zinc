//!
//! The semantic analyzer integer value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    TypesMismatchEquals(String, String),
    TypesMismatchNotEquals(String, String),
    TypesMismatchGreaterEquals(String, String),
    TypesMismatchLesserEquals(String, String),
    TypesMismatchGreater(String, String),
    TypesMismatchLesser(String, String),
    TypesMismatchAddition(String, String),
    TypesMismatchSubtraction(String, String),
    TypesMismatchMultiplication(String, String),
    TypesMismatchDivision(String, String),
    TypesMismatchRemainder(String, String),

    ForbiddenFieldRemainder,
    ForbiddenFieldNegation,
}
