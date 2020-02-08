//!
//! The semantic analyzer element error.
//!

use failure::Fail;

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::value::error::Error as ValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Place(PlaceError),
    #[fail(display = "{}", _0)]
    Value(ValueError),
    #[fail(display = "{}", _0)]
    Constant(ConstantError),

    #[fail(
        display = "'=' operator expected a place expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAssignmentFirstOperandExpectedPlace(String),
    #[fail(
        display = "'=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAssignmentSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'..=' operator expected a constant expression as the first operand, but got '{}'",
        _0
    )]
    OperatorRangeInclusiveFirstOperandExpectedConstant(String),
    #[fail(
        display = "'..=' operator expected a constant expression as the second operand, but got '{}'",
        _0
    )]
    OperatorRangeInclusiveSecondOperandExpectedConstant(String),
    #[fail(
        display = "'..' operator expected a constant expression as the first operand, but got '{}'",
        _0
    )]
    OperatorRangeFirstOperandExpectedConstant(String),
    #[fail(
        display = "'..' operator expected a constant expression as the second operand, but got '{}'",
        _0
    )]
    OperatorRangeSecondOperandExpectedConstant(String),

    #[fail(
        display = "'||' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'||' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'^^' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'^^' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'&&' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'&&' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'==' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'==' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'!=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'!=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'>=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'>=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'<=' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'<=' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'>' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'>' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'<' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'<' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'+' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'+' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'-' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'-' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'*' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'*' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'/' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'/' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'%' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'%' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'as' operator expected an evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorCastingFirstOperandExpectedEvaluable(String),
    #[fail(
        display = "'as' operator expected a type expression as the second operand, but got '{}'",
        _0
    )]
    OperatorCastingSecondOperandExpectedType(String),

    #[fail(
        display = "unary '-' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedEvaluable(String),
    #[fail(
        display = "'!' operator expected an evaluable expression as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedEvaluable(String),

    #[fail(
        display = "'[]' operator expected a place or evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedPlaceOrEvaluable(String),
    #[fail(
        display = "'[]' operator expected an evaluable expression as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedEvaluable(String),

    #[fail(
        display = "'.' operator expected a place or evaluable expression as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedPlaceOrEvaluable(String),
    #[fail(
        display = "'.' operator expected a member identifier as the second operand, but got '{}'",
        _0
    )]
    OperatorFieldSecondOperandExpectedMember(String),

    #[fail(
        display = "'::' operator expected a path expression as the first operand, but got '{}'",
        _0
    )]
    OperatorPathFirstOperandExpectedPath(String),
    #[fail(
        display = "'::' operator expected a member string as the second operand, but got '{}'",
        _0
    )]
    OperatorPathSecondOperandExpectedMemberString(String),
}
