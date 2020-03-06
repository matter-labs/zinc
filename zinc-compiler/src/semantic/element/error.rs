//!
//! The semantic analyzer element error.
//!

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::value::error::Error as ValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorAssignmentFirstOperandExpectedPlace(String),
    OperatorAssignmentSecondOperandExpectedEvaluable(String),
    OperatorAssignmentAdditionFirstOperandExpectedPlace(String),
    OperatorAssignmentAdditionSecondOperandExpectedEvaluable(String),
    OperatorAssignmentSubtractionFirstOperandExpectedPlace(String),
    OperatorAssignmentSubtractionSecondOperandExpectedEvaluable(String),
    OperatorAssignmentMultiplicationFirstOperandExpectedPlace(String),
    OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable(String),
    OperatorAssignmentDivisionFirstOperandExpectedPlace(String),
    OperatorAssignmentDivisionSecondOperandExpectedEvaluable(String),
    OperatorAssignmentRemainderFirstOperandExpectedPlace(String),
    OperatorAssignmentRemainderSecondOperandExpectedEvaluable(String),

    OperatorRangeInclusiveFirstOperandExpectedConstant(String),
    OperatorRangeInclusiveSecondOperandExpectedConstant(String),
    OperatorRangeFirstOperandExpectedConstant(String),
    OperatorRangeSecondOperandExpectedConstant(String),

    OperatorOrFirstOperandExpectedEvaluable(String),
    OperatorOrSecondOperandExpectedEvaluable(String),

    OperatorXorFirstOperandExpectedEvaluable(String),
    OperatorXorSecondOperandExpectedEvaluable(String),

    OperatorAndFirstOperandExpectedEvaluable(String),
    OperatorAndSecondOperandExpectedEvaluable(String),

    OperatorEqualsFirstOperandExpectedEvaluable(String),
    OperatorEqualsSecondOperandExpectedEvaluable(String),

    OperatorNotEqualsFirstOperandExpectedEvaluable(String),
    OperatorNotEqualsSecondOperandExpectedEvaluable(String),

    OperatorGreaterEqualsFirstOperandExpectedEvaluable(String),
    OperatorGreaterEqualsSecondOperandExpectedEvaluable(String),

    OperatorLesserEqualsFirstOperandExpectedEvaluable(String),
    OperatorLesserEqualsSecondOperandExpectedEvaluable(String),

    OperatorGreaterFirstOperandExpectedEvaluable(String),
    OperatorGreaterSecondOperandExpectedEvaluable(String),

    OperatorLesserFirstOperandExpectedEvaluable(String),
    OperatorLesserSecondOperandExpectedEvaluable(String),

    OperatorAdditionFirstOperandExpectedEvaluable(String),
    OperatorAdditionSecondOperandExpectedEvaluable(String),

    OperatorSubtractionFirstOperandExpectedEvaluable(String),
    OperatorSubtractionSecondOperandExpectedEvaluable(String),

    OperatorMultiplicationFirstOperandExpectedEvaluable(String),
    OperatorMultiplicationSecondOperandExpectedEvaluable(String),

    OperatorDivisionFirstOperandExpectedEvaluable(String),
    OperatorDivisionSecondOperandExpectedEvaluable(String),

    OperatorRemainderFirstOperandExpectedEvaluable(String),
    OperatorRemainderSecondOperandExpectedEvaluable(String),

    OperatorCastingFirstOperandExpectedEvaluable(String),
    OperatorCastingSecondOperandExpectedType(String),

    OperatorNegationExpectedEvaluable(String),

    OperatorNotExpectedEvaluable(String),

    OperatorIndexFirstOperandExpectedPlaceOrEvaluable(String),
    OperatorIndexSecondOperandExpectedEvaluable(String),

    OperatorFieldFirstOperandExpectedPlaceOrEvaluable(String),
    OperatorFieldSecondOperandExpectedMember(String),

    OperatorPathFirstOperandExpectedPath(String),
    OperatorPathSecondOperandExpectedMemberString(String),

    Place(PlaceError),
    Value(ValueError),
    Constant(ConstantError),
}
