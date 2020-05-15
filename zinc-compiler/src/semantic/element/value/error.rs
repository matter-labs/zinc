//!
//! The semantic analyzer value element error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorOrFirstOperandExpectedBoolean {
        location: Location,
        found: String,
    },
    OperatorOrSecondOperandExpectedBoolean {
        location: Location,
        found: String,
    },

    OperatorXorFirstOperandExpectedBoolean {
        location: Location,
        found: String,
    },
    OperatorXorSecondOperandExpectedBoolean {
        location: Location,
        found: String,
    },

    OperatorAndFirstOperandExpectedBoolean {
        location: Location,
        found: String,
    },
    OperatorAndSecondOperandExpectedBoolean {
        location: Location,
        found: String,
    },

    OperatorEqualsSecondOperandExpectedUnit {
        location: Location,
        found: String,
    },
    OperatorEqualsSecondOperandExpectedBoolean {
        location: Location,
        found: String,
    },
    OperatorEqualsSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorEqualsFirstOperandExpectedPrimitiveType {
        location: Location,
        found: String,
    },

    OperatorNotEqualsSecondOperandExpectedUnit {
        location: Location,
        found: String,
    },
    OperatorNotEqualsSecondOperandExpectedBoolean {
        location: Location,
        found: String,
    },
    OperatorNotEqualsSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorNotEqualsFirstOperandExpectedPrimitiveType {
        location: Location,
        found: String,
    },

    OperatorGreaterEqualsFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorGreaterEqualsSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorLesserEqualsFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorLesserEqualsSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorGreaterFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorGreaterSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorLesserFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorLesserSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorBitwiseOrFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorBitwiseOrSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorBitwiseXorFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorBitwiseXorSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorBitwiseAndFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorBitwiseAndSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorBitwiseShiftRightFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorBitwiseShiftRightSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorAdditionFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorAdditionSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorSubtractionFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorSubtractionSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorMultiplicationFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorMultiplicationSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorDivisionFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorDivisionSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorRemainderFirstOperandExpectedInteger {
        location: Location,
        found: String,
    },
    OperatorRemainderSecondOperandExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorNotExpectedBoolean {
        location: Location,
        found: String,
    },

    OperatorBitwiseNotExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorNegationExpectedInteger {
        location: Location,
        found: String,
    },

    OperatorIndexFirstOperandExpectedArray {
        location: Location,
        found: String,
    },
    OperatorIndexSecondOperandExpectedIntegerOrRange {
        location: Location,
        found: String,
    },

    OperatorDotFirstOperandExpectedTuple {
        location: Location,
        found: String,
    },
    OperatorDotFirstOperandExpectedStructure {
        location: Location,
        found: String,
    },

    Integer(IntegerValueError),
    Array(ArrayValueError),
    Tuple(TupleValueError),
    Structure(StructureValueError),
    Casting {
        location: Location,
        inner: CastingError,
        reference: Option<Location>,
    },
}
