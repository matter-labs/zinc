//!
//! The semantic analyzer value element error.
//!

use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
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

    Integer(IntegerValueError),
    Array(ArrayValueError),
    Tuple(TupleValueError),
    Structure(StructureValueError),
    Casting(CastingError),
}
