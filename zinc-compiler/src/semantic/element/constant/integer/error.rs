//!
//! The semantic analyzer constant integer element error.
//!

use failure::Fail;
use num_bigint::BigInt;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'==' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchEquals(String, String),
    #[fail(
        display = "'!=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchNotEquals(String, String),
    #[fail(
        display = "'>=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchGreaterEquals(String, String),
    #[fail(
        display = "'<=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchLesserEquals(String, String),
    #[fail(display = "'>' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchGreater(String, String),
    #[fail(display = "'<' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchLesser(String, String),
    #[fail(display = "'+' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchAddition(String, String),
    #[fail(display = "'-' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchSubtraction(String, String),
    #[fail(display = "'*' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchMultiplication(String, String),
    #[fail(display = "'/' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchDivision(String, String),
    #[fail(display = "'%' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    TypesMismatchRemainder(String, String),

    #[fail(
        display = "'+' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowAddition(BigInt, String),
    #[fail(
        display = "'-' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowSubtraction(BigInt, String),
    #[fail(
        display = "'*' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowMultiplication(BigInt, String),
    #[fail(
        display = "'/' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowDivision(BigInt, String),
    #[fail(
        display = "'%' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowRemainder(BigInt, String),
    #[fail(
        display = "'as' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowCasting(BigInt, String),
    #[fail(
        display = "unary '-' operator overflow: '{}' cannot be represented by type '{}'",
        _0, _1
    )]
    OverflowNegation(BigInt, String),

    #[fail(display = "negative value '{}' of unsigned type '{}'", _0, _1)]
    UnsignedNegative(BigInt, String),

    #[fail(display = "'/' operator division by zero")]
    ZeroDivision,
    #[fail(display = "'%' operator division by zero")]
    ZeroRemainder,

    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    LiteralTooLargeForIndex(String, usize),
    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    IntegerTooLargeForField(String, usize),

    #[fail(display = "'%' operator is temporarily forbidden for field elements")]
    ForbiddenFieldRemainder,
    #[fail(display = "unary '-' operator is forbidden for field elements")]
    ForbiddenFieldNegation,
}
