//!
//! The semantic analyzer constant integer element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'..=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchRangeInclusive(String, String),
    #[fail(
        display = "'..' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    TypesMismatchRange(String, String),
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

    #[fail(display = "integer bitlength is too big for negation: {}", _0)]
    NegationBitlengthTooBig(usize),

    #[fail(display = "'/' operator division by zero")]
    DivisionZero,
    #[fail(display = "'%' operator division by zero")]
    RemainderZero,

    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    LiteralTooLargeForIndex(String, usize),
    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    IntegerTooLargeForField(String, usize),
}
