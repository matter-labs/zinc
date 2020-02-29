//!
//! The semantic analyzer constant integer element error.
//!

use num_bigint::BigInt;

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

    OverflowAddition(BigInt, String),
    OverflowSubtraction(BigInt, String),
    OverflowMultiplication(BigInt, String),
    OverflowDivision(BigInt, String),
    OverflowRemainder(BigInt, String),
    OverflowCasting(BigInt, String),
    OverflowNegation(BigInt, String),

    ForbiddenFieldRemainder,
    ForbiddenFieldNegation,

    ZeroDivision,
    ZeroRemainder,

    IntegerTooLarge(String, usize),
    UnsignedNegative(BigInt, String),
}
