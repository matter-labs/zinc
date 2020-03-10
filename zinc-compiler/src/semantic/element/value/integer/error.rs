//!
//! The semantic analyzer integer value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    TypesMismatchEquals { first: String, second: String },
    TypesMismatchNotEquals { first: String, second: String },
    TypesMismatchGreaterEquals { first: String, second: String },
    TypesMismatchLesserEquals { first: String, second: String },
    TypesMismatchGreater { first: String, second: String },
    TypesMismatchLesser { first: String, second: String },
    TypesMismatchAddition { first: String, second: String },
    TypesMismatchSubtraction { first: String, second: String },
    TypesMismatchMultiplication { first: String, second: String },
    TypesMismatchDivision { first: String, second: String },
    TypesMismatchRemainder { first: String, second: String },

    ForbiddenFieldDivision,
    ForbiddenFieldRemainder,
    ForbiddenFieldNegation,
}
