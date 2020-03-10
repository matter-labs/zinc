//!
//! The semantic analyzer constant element error.
//!

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorRangeInclusiveFirstOperandExpectedInteger { found: String },
    OperatorRangeInclusiveSecondOperandExpectedInteger { found: String },
    OperatorRangeFirstOperandExpectedInteger { found: String },
    OperatorRangeSecondOperandExpectedInteger { found: String },

    OperatorOrFirstOperandExpectedBoolean { found: String },
    OperatorOrSecondOperandExpectedBoolean { found: String },

    OperatorXorFirstOperandExpectedBoolean { found: String },
    OperatorXorSecondOperandExpectedBoolean { found: String },

    OperatorAndFirstOperandExpectedBoolean { found: String },
    OperatorAndSecondOperandExpectedBoolean { found: String },

    OperatorEqualsSecondOperandExpectedUnit { found: String },
    OperatorEqualsSecondOperandExpectedBoolean { found: String },
    OperatorEqualsSecondOperandExpectedInteger { found: String },
    OperatorEqualsFirstOperandExpectedPrimitiveType { found: String },

    OperatorNotEqualsSecondOperandExpectedUnit { found: String },
    OperatorNotEqualsSecondOperandExpectedBoolean { found: String },
    OperatorNotEqualsSecondOperandExpectedInteger { found: String },
    OperatorNotEqualsFirstOperandExpectedPrimitiveType { found: String },

    OperatorGreaterEqualsFirstOperandExpectedInteger { found: String },
    OperatorGreaterEqualsSecondOperandExpectedInteger { found: String },

    OperatorLesserEqualsFirstOperandExpectedInteger { found: String },
    OperatorLesserEqualsSecondOperandExpectedInteger { found: String },

    OperatorGreaterFirstOperandExpectedInteger { found: String },
    OperatorGreaterSecondOperandExpectedInteger { found: String },

    OperatorLesserFirstOperandExpectedInteger { found: String },
    OperatorLesserSecondOperandExpectedInteger { found: String },

    OperatorAdditionFirstOperandExpectedInteger { found: String },
    OperatorAdditionSecondOperandExpectedInteger { found: String },

    OperatorSubtractionFirstOperandExpectedInteger { found: String },
    OperatorSubtractionSecondOperandExpectedInteger { found: String },

    OperatorMultiplicationFirstOperandExpectedInteger { found: String },
    OperatorMultiplicationSecondOperandExpectedInteger { found: String },

    OperatorDivisionFirstOperandExpectedInteger { found: String },
    OperatorDivisionSecondOperandExpectedInteger { found: String },

    OperatorRemainderFirstOperandExpectedInteger { found: String },
    OperatorRemainderSecondOperandExpectedInteger { found: String },

    OperatorNegationExpectedInteger { found: String },
    OperatorNotExpectedBoolean { found: String },

    Integer(IntegerConstantError),
    Casting(CasterError),
}
