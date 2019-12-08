//!
//! The semantic analyzer integer value element error.
//!

use failure::Fail;

use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
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
    #[fail(display = "'%' operator operand types mismatch: '{}' and '{}'", _0, _1)]
    OperatorRemainderOperandTypesMismatch(Type, Type),

    #[fail(display = "integer bitlength is too big for negation: {}", _0)]
    Negation(usize),
}
