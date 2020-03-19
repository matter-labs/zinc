//!
//! The semantic analyzer element error.
//!

use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::value::error::Error as ValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorAssignmentFirstOperandExpectedPlace { found: String },
    OperatorAssignmentSecondOperandExpectedEvaluable { found: String },
    OperatorAssignmentAdditionFirstOperandExpectedPlace { found: String },
    OperatorAssignmentAdditionSecondOperandExpectedEvaluable { found: String },
    OperatorAssignmentSubtractionFirstOperandExpectedPlace { found: String },
    OperatorAssignmentSubtractionSecondOperandExpectedEvaluable { found: String },
    OperatorAssignmentMultiplicationFirstOperandExpectedPlace { found: String },
    OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable { found: String },
    OperatorAssignmentDivisionFirstOperandExpectedPlace { found: String },
    OperatorAssignmentDivisionSecondOperandExpectedEvaluable { found: String },
    OperatorAssignmentRemainderFirstOperandExpectedPlace { found: String },
    OperatorAssignmentRemainderSecondOperandExpectedEvaluable { found: String },

    OperatorRangeInclusiveFirstOperandExpectedConstant { found: String },
    OperatorRangeInclusiveSecondOperandExpectedConstant { found: String },
    OperatorRangeFirstOperandExpectedConstant { found: String },
    OperatorRangeSecondOperandExpectedConstant { found: String },

    OperatorOrFirstOperandExpectedEvaluable { found: String },
    OperatorOrSecondOperandExpectedEvaluable { found: String },

    OperatorXorFirstOperandExpectedEvaluable { found: String },
    OperatorXorSecondOperandExpectedEvaluable { found: String },

    OperatorAndFirstOperandExpectedEvaluable { found: String },
    OperatorAndSecondOperandExpectedEvaluable { found: String },

    OperatorEqualsFirstOperandExpectedEvaluable { found: String },
    OperatorEqualsSecondOperandExpectedEvaluable { found: String },

    OperatorNotEqualsFirstOperandExpectedEvaluable { found: String },
    OperatorNotEqualsSecondOperandExpectedEvaluable { found: String },

    OperatorGreaterEqualsFirstOperandExpectedEvaluable { found: String },
    OperatorGreaterEqualsSecondOperandExpectedEvaluable { found: String },

    OperatorLesserEqualsFirstOperandExpectedEvaluable { found: String },
    OperatorLesserEqualsSecondOperandExpectedEvaluable { found: String },

    OperatorGreaterFirstOperandExpectedEvaluable { found: String },
    OperatorGreaterSecondOperandExpectedEvaluable { found: String },

    OperatorLesserFirstOperandExpectedEvaluable { found: String },
    OperatorLesserSecondOperandExpectedEvaluable { found: String },

    OperatorBitwiseOrFirstOperandExpectedEvaluable { found: String },
    OperatorBitwiseOrSecondOperandExpectedEvaluable { found: String },

    OperatorBitwiseXorFirstOperandExpectedEvaluable { found: String },
    OperatorBitwiseXorSecondOperandExpectedEvaluable { found: String },

    OperatorBitwiseAndFirstOperandExpectedEvaluable { found: String },
    OperatorBitwiseAndSecondOperandExpectedEvaluable { found: String },

    OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable { found: String },
    OperatorBitwiseShiftLeftSecondOperandExpectedConstant { found: String },

    OperatorBitwiseShiftRightFirstOperandExpectedEvaluable { found: String },
    OperatorBitwiseShiftRightSecondOperandExpectedConstant { found: String },

    OperatorAdditionFirstOperandExpectedEvaluable { found: String },
    OperatorAdditionSecondOperandExpectedEvaluable { found: String },

    OperatorSubtractionFirstOperandExpectedEvaluable { found: String },
    OperatorSubtractionSecondOperandExpectedEvaluable { found: String },

    OperatorMultiplicationFirstOperandExpectedEvaluable { found: String },
    OperatorMultiplicationSecondOperandExpectedEvaluable { found: String },

    OperatorDivisionFirstOperandExpectedEvaluable { found: String },
    OperatorDivisionSecondOperandExpectedEvaluable { found: String },

    OperatorRemainderFirstOperandExpectedEvaluable { found: String },
    OperatorRemainderSecondOperandExpectedEvaluable { found: String },

    OperatorCastingFirstOperandExpectedEvaluable { found: String },
    OperatorCastingSecondOperandExpectedType { found: String },

    OperatorNotExpectedEvaluable { found: String },

    OperatorBitwiseNotExpectedEvaluable { found: String },

    OperatorNegationExpectedEvaluable { found: String },

    OperatorIndexFirstOperandExpectedPlaceOrEvaluable { found: String },
    OperatorIndexSecondOperandExpectedEvaluable { found: String },

    OperatorFieldFirstOperandExpectedPlaceOrEvaluable { found: String },
    OperatorFieldSecondOperandExpectedMember { found: String },

    OperatorPathFirstOperandExpectedPath { found: String },
    OperatorPathSecondOperandExpectedMemberString { found: String },

    Place(PlaceError),
    Value(ValueError),
    Constant(ConstantError),
}
