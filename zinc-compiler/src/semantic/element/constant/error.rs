//!
//! The semantic analyzer constant element error.
//!

use failure::Fail;

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::Constant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'..=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRangeInclusiveFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'..=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRangeInclusiveSecondOperandExpectedInteger(Constant),
    #[fail(
        display = "'..' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRangeFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'..' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRangeSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'||' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedBoolean(Constant),
    #[fail(
        display = "'||' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedBoolean(Constant),

    #[fail(
        display = "'^^' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedBoolean(Constant),
    #[fail(
        display = "'^^' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedBoolean(Constant),

    #[fail(
        display = "'&&' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedBoolean(Constant),
    #[fail(
        display = "'&&' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedBoolean(Constant),

    #[fail(
        display = "'==' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedUnit(Constant),
    #[fail(
        display = "'==' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedBoolean(Constant),
    #[fail(
        display = "'==' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedInteger(Constant),
    #[fail(
        display = "'==' operator expected a primitive type constant as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedPrimitiveType(Constant),

    #[fail(
        display = "'!=' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedUnit(Constant),
    #[fail(
        display = "'!=' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedBoolean(Constant),
    #[fail(
        display = "'!=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedInteger(Constant),
    #[fail(
        display = "'!=' operator expected a primitive type constant as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedPrimitiveType(Constant),

    #[fail(
        display = "'>=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'>=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'<=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'<=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'>' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'>' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'<' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'<' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'+' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'+' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'-' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'-' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'*' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'*' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'/' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'/' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "'%' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedInteger(Constant),
    #[fail(
        display = "'%' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedInteger(Constant),

    #[fail(
        display = "unary '-' operator expected an integer as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedInteger(Constant),
    #[fail(
        display = "'!' operator expected a boolean as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedBoolean(Constant),

    #[fail(display = "{}", _0)]
    Integer(IntegerConstantError),
    #[fail(display = "{}", _0)]
    Casting(CasterError),
}
