//!
//! The semantic analyzer constant element error.
//!

use failure::Fail;

use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'==' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    OperatorEqualsOperandTypesMismatch(Type, Type),
    #[fail(
        display = "'!=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    OperatorNotEqualsOperandTypesMismatch(Type, Type),
    #[fail(
        display = "'>=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    OperatorGreaterEqualsOperandTypesMismatch(Type, Type),
    #[fail(
        display = "'<=' operator operand types mismatch: '{}' and '{}'",
        _0, _1
    )]
    OperatorLesserEqualsOperandTypesMismatch(Type, Type),
    #[fail(display = "'>' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorGreaterOperandTypesMismatch(Type, Type),
    #[fail(display = "'<' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorLesserOperandTypesMismatch(Type, Type),
    #[fail(display = "'+' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorAdditionOperandTypesMismatch(Type, Type),
    #[fail(display = "'-' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorSubtractionOperandTypesMismatch(Type, Type),
    #[fail(display = "'*' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorMultiplicationOperandTypesMismatch(Type, Type),
    #[fail(display = "'/' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorDivisionOperandTypesMismatch(Type, Type),
    #[fail(display = "'/' operator division by zero")]
    OperatorDivisionZero,
    #[fail(display = "'%' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorRemainderOperandTypesMismatch(Type, Type),
    #[fail(display = "'%' operator division by zero")]
    OperatorRemainderZero,

    #[fail(display = "integer bitlength is too big for negation: {}", _0)]
    OperatorNegationBitlengthTooBig(usize),

    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    LiteralTooLargeForIndex(String, usize),
    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    LiteralTooLargeForField(String, usize),
}
