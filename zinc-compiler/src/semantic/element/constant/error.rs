//!
//! The semantic analyzer constant element error.
//!

use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::constant::array::error::Error as ArrayConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::structure::error::Error as StructureConstantError;
use crate::semantic::element::constant::tuple::error::Error as TupleConstantError;

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

    OperatorBitwiseOrFirstOperandExpectedInteger { found: String },
    OperatorBitwiseOrSecondOperandExpectedInteger { found: String },

    OperatorBitwiseXorFirstOperandExpectedInteger { found: String },
    OperatorBitwiseXorSecondOperandExpectedInteger { found: String },

    OperatorBitwiseAndFirstOperandExpectedInteger { found: String },
    OperatorBitwiseAndSecondOperandExpectedInteger { found: String },

    OperatorBitwiseShiftLeftFirstOperandExpectedInteger { found: String },
    OperatorBitwiseShiftLeftSecondOperandExpectedInteger { found: String },

    OperatorBitwiseShiftRightFirstOperandExpectedInteger { found: String },
    OperatorBitwiseShiftRightSecondOperandExpectedInteger { found: String },

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

    OperatorNotExpectedBoolean { found: String },

    OperatorBitwiseNotExpectedInteger { found: String },

    OperatorNegationExpectedInteger { found: String },

    OperatorIndexFirstOperandExpectedArray { found: String },
    OperatorIndexSecondOperandExpectedIntegerOrRange { found: String },

    OperatorFieldFirstOperandExpectedTuple { found: String },
    OperatorFieldFirstOperandExpectedStructure { found: String },

    Integer(IntegerConstantError),
    Array(ArrayConstantError),
    Tuple(TupleConstantError),
    Structure(StructureConstantError),
    Casting(CastingError),
}
