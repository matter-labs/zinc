//!
//! The semantic analyzer constant element error.
//!

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorRangeInclusiveFirstOperandExpectedInteger(String),
    OperatorRangeInclusiveSecondOperandExpectedInteger(String),
    OperatorRangeFirstOperandExpectedInteger(String),
    OperatorRangeSecondOperandExpectedInteger(String),

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

    Integer(IntegerConstantError),
    Casting(CasterError),
}
