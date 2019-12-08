//!
//! The semantic analyzer value element error.
//!

use failure::Fail;

use crate::semantic::CasterError;
use crate::semantic::IntegerValueError;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'||' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedBoolean(Type),
    #[fail(
        display = "'||' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedBoolean(Type),

    #[fail(
        display = "'^^' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedBoolean(Type),
    #[fail(
        display = "'^^' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedBoolean(Type),

    #[fail(
        display = "'&&' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedBoolean(Type),
    #[fail(
        display = "'&&' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedBoolean(Type),

    #[fail(
        display = "'==' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedUnit(Type),
    #[fail(
        display = "'==' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedBoolean(Type),
    #[fail(
        display = "'==' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedInteger(Type),
    #[fail(
        display = "'==' operator expected two primitive type constants, but got '{}' and '{}'",
        _0, _1
    )]
    OperatorEqualsExpectedPrimitiveTypes(Type, Type),

    #[fail(
        display = "'!=' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedUnit(Type),
    #[fail(
        display = "'!=' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedBoolean(Type),
    #[fail(
        display = "'!=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedInteger(Type),
    #[fail(
        display = "'!=' operator expected two primitive type constants, but got '{}' and '{}'",
        _0, _1
    )]
    OperatorNotEqualsExpectedPrimitiveTypes(Type, Type),

    #[fail(
        display = "'>=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'>=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'<=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'<=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'>' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'>' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'<' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'<' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'+' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'+' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'-' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'-' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'*' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'*' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'/' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'/' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedInteger(Type),

    #[fail(
        display = "'%' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedInteger(Type),
    #[fail(
        display = "'%' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedInteger(Type),

    #[fail(
        display = "unary '-' operator expected an integer as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedInteger(Type),
    #[fail(
        display = "'!' operator expected a boolean as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedInteger(Type),

    #[fail(display = "integer: {}", _0)]
    Integer(IntegerValueError),
    #[fail(display = "casting: {}", _0)]
    Casting(CasterError),
}
