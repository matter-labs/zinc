//!
//! The semantic analyzer value element error.
//!

use failure::Fail;

use crate::semantic::CasterError;
use crate::semantic::IntegerValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'||' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'||' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'^^' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'^^' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'&&' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'&&' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'==' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedUnit(String),
    #[fail(
        display = "'==' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedBoolean(String),
    #[fail(
        display = "'==' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedInteger(String),
    #[fail(
        display = "'==' operator expected a primitive type as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedPrimitiveType(String),

    #[fail(
        display = "'!=' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedUnit(String),
    #[fail(
        display = "'!=' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedBoolean(String),
    #[fail(
        display = "'!=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedInteger(String),
    #[fail(
        display = "'!=' operator expected a primitive type as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedPrimitiveType(String),

    #[fail(
        display = "'>=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedInteger(String),
    #[fail(
        display = "'>=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedInteger(String),

    #[fail(
        display = "'<=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedInteger(String),
    #[fail(
        display = "'<=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedInteger(String),

    #[fail(
        display = "'>' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedInteger(String),
    #[fail(
        display = "'>' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedInteger(String),

    #[fail(
        display = "'<' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedInteger(String),
    #[fail(
        display = "'<' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedInteger(String),

    #[fail(
        display = "'+' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'+' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'-' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'-' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'*' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedInteger(String),
    #[fail(
        display = "'*' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedInteger(String),

    #[fail(
        display = "'/' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'/' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'%' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedInteger(String),
    #[fail(
        display = "'%' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedInteger(String),

    #[fail(
        display = "unary '-' operator expected an integer as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedInteger(String),
    #[fail(
        display = "'!' operator expected a boolean as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedBoolean(String),

    #[fail(display = "integer: {}", _0)]
    Integer(IntegerValueError),
    #[fail(display = "casting: {}", _0)]
    Casting(CasterError),
}
