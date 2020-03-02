//!
//! The semantic analyzer value element error.
//!

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorOrFirstOperandExpectedBoolean(String),
    OperatorOrSecondOperandExpectedBoolean(String),

    OperatorXorFirstOperandExpectedBoolean(String),
    OperatorXorSecondOperandExpectedBoolean(String),

    OperatorAndFirstOperandExpectedBoolean(String),
    OperatorAndSecondOperandExpectedBoolean(String),

    OperatorEqualsSecondOperandExpectedUnit(String),
    OperatorEqualsSecondOperandExpectedBoolean(String),
    OperatorEqualsSecondOperandExpectedInteger(String),
    OperatorEqualsFirstOperandExpectedPrimitiveType(String),

    OperatorNotEqualsSecondOperandExpectedUnit(String),
    OperatorNotEqualsSecondOperandExpectedBoolean(String),
    OperatorNotEqualsSecondOperandExpectedInteger(String),
    OperatorNotEqualsFirstOperandExpectedPrimitiveType(String),

    OperatorGreaterEqualsFirstOperandExpectedInteger(String),
    OperatorGreaterEqualsSecondOperandExpectedInteger(String),

    OperatorLesserEqualsFirstOperandExpectedInteger(String),
    OperatorLesserEqualsSecondOperandExpectedInteger(String),

    OperatorGreaterFirstOperandExpectedInteger(String),
    OperatorGreaterSecondOperandExpectedInteger(String),

    OperatorLesserFirstOperandExpectedInteger(String),
    OperatorLesserSecondOperandExpectedInteger(String),

    OperatorAdditionFirstOperandExpectedInteger(String),
    OperatorAdditionSecondOperandExpectedInteger(String),

    OperatorSubtractionFirstOperandExpectedInteger(String),
    OperatorSubtractionSecondOperandExpectedInteger(String),

    OperatorMultiplicationFirstOperandExpectedInteger(String),
    OperatorMultiplicationSecondOperandExpectedInteger(String),

    OperatorDivisionFirstOperandExpectedInteger(String),
    OperatorDivisionSecondOperandExpectedInteger(String),

    OperatorRemainderFirstOperandExpectedInteger(String),
    OperatorRemainderSecondOperandExpectedInteger(String),

    OperatorNegationExpectedInteger(String),

    OperatorNotExpectedBoolean(String),

    OperatorIndexFirstOperandExpectedArray(String),
    OperatorIndexSecondOperandExpectedIntegerOrRange(String),

    OperatorFieldFirstOperandExpectedTuple(String),
    OperatorFieldFirstOperandExpectedStructure(String),

    ConvertingFromType(String),

    Integer(IntegerValueError),
    Array(ArrayValueError),
    Tuple(TupleValueError),
    Structure(StructureValueError),
    Casting(CasterError),
}
