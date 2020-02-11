//!
//! The semantic analyzer integer value element error.
//!

use failure::Fail;

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

    #[fail(display = "'>=' operator is temporarily forbidden for field elements")]
    ForbiddenFieldGreaterEquals,
    #[fail(display = "'<=' operator is temporarily forbidden for field elements")]
    ForbiddenFieldLesserEquals,
    #[fail(display = "'>' operator is temporarily forbidden for field elements")]
    ForbiddenFieldGreater,
    #[fail(display = "'<' operator is temporarily forbidden for field elements")]
    ForbiddenFieldLesser,
    #[fail(display = "'/' operator is temporarily forbidden for field elements")]
    ForbiddenFieldDivision,
    #[fail(display = "'%' operator is temporarily forbidden for field elements")]
    ForbiddenFieldRemainder,
    #[fail(display = "unary '-' operator is forbidden for field elements")]
    ForbiddenFieldNegation,
}
