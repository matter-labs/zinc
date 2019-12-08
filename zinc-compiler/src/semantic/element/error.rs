//!
//! The semantic analyzer element error.
//!

use failure::Fail;

use crate::semantic::ConstantError;
use crate::semantic::Element;
use crate::semantic::PlaceError;
use crate::semantic::ValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "place: {}", _0)]
    Place(PlaceError),
    #[fail(display = "value: {}", _0)]
    Value(ValueError),
    #[fail(display = "constant: {}", _0)]
    Constant(ConstantError),

    #[fail(
        display = "'=' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAssignmentFirstOperandExpectedPlace(Element),
    #[fail(
        display = "'=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAssignmentSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'||' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'||' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'^^' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'^^' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'&&' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'&&' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'==' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'==' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'!=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'!=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'>=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'>=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'<=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'<=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'>' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'>' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'<' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'<' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'+' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'+' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'-' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'-' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'*' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'*' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'/' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'/' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'%' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'%' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedEvaluable(Element),

    #[fail(
        display = "'as' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorCastingFirstOperandExpectedEvaluable(Element),
    #[fail(
        display = "'as' operator expected a type expression as the second operand, but got '{}'",
        _0
    )]
    OperatorCastingSecondOperandExpectedType(Element),

    #[fail(
        display = "unary '-' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedEvaluable(Element),
    #[fail(
        display = "'!' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedEvaluable(Element),

    #[fail(
        display = "'[]' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedPlace(Element),
    #[fail(
        display = "'[]' operator expected a constant expression as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedIntegerConstant(Element),

    #[fail(
        display = "'.' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldAccessFirstOperandExpectedPlace(Element),
    #[fail(
        display = "'.' operator expected a member identifier as the second operand, but got '{}'",
        _0
    )]
    OperatorFieldAccessSecondOperandExpectedMember(Element),
}
