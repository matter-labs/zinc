//!
//! The semantic analyzer element error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::value::error::Error as ValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorAssignmentFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentAdditionFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentSubtractionFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentDivisionFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAssignmentRemainderFirstOperandExpectedPlace {
        location: Location,
        found: String,
    },
    OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorRangeInclusiveFirstOperandExpectedConstant {
        location: Location,
        found: String,
    },
    OperatorRangeInclusiveSecondOperandExpectedConstant {
        location: Location,
        found: String,
    },
    OperatorRangeFirstOperandExpectedConstant {
        location: Location,
        found: String,
    },
    OperatorRangeSecondOperandExpectedConstant {
        location: Location,
        found: String,
    },

    OperatorOrFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorOrSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorXorFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorXorSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorAndFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAndSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorEqualsFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorEqualsSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorNotEqualsFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorNotEqualsSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorGreaterEqualsFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorGreaterEqualsSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorLesserEqualsFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorLesserEqualsSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorGreaterFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorGreaterSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorLesserFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorLesserSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorBitwiseOrFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorBitwiseOrSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorBitwiseXorFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorBitwiseXorSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorBitwiseAndFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorBitwiseAndSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
        location: Location,
        found: String,
    },

    OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorBitwiseShiftRightSecondOperandExpectedConstant {
        location: Location,
        found: String,
    },

    OperatorAdditionFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorAdditionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorSubtractionFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorSubtractionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorMultiplicationFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorMultiplicationSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorDivisionFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorDivisionSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorRemainderFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorRemainderSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorCastingFirstOperandExpectedEvaluable {
        location: Location,
        found: String,
    },
    OperatorCastingSecondOperandExpectedType {
        location: Location,
        found: String,
    },

    OperatorNotExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorBitwiseNotExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorNegationExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
        location: Location,
        found: String,
    },
    OperatorIndexSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    OperatorDotFirstOperandExpectedPlaceOrEvaluable {
        location: Location,
        found: String,
    },
    OperatorDotSecondOperandExpectedIdentifier {
        location: Location,
        found: String,
    },

    OperatorPathFirstOperandExpectedPath {
        location: Location,
        found: String,
    },
    OperatorPathSecondOperandExpectedIdentifier {
        location: Location,
        found: String,
    },

    OperatorStructureFirstOperandExpectedType {
        location: Location,
        found: String,
    },
    OperatorStructureSecondOperandExpectedEvaluable {
        location: Location,
        found: String,
    },

    Place(PlaceError),
    Value(ValueError),
    Constant(ConstantError),
    Type(TypeError),
}
