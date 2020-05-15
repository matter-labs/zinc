//!
//! The semantic analyzer integer value element error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    TypesMismatchEquals {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchNotEquals {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchGreaterEquals {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchLesserEquals {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchGreater {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchLesser {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchBitwiseOr {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchBitwiseXor {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchBitwiseAnd {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchAddition {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchSubtraction {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchMultiplication {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchDivision {
        location: Location,
        first: String,
        second: String,
    },
    TypesMismatchRemainder {
        location: Location,
        first: String,
        second: String,
    },

    OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
        location: Location,
        found: String,
    },
    OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
        location: Location,
        found: String,
    },

    ForbiddenFieldDivision {
        location: Location,
    },
    ForbiddenFieldRemainder {
        location: Location,
    },
    ForbiddenSignedBitwise {
        location: Location,
    },
    ForbiddenFieldBitwise {
        location: Location,
    },
    ForbiddenFieldNegation {
        location: Location,
    },
}
