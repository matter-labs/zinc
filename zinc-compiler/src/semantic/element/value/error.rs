//!
//! The semantic analyzer value element error.
//!

use failure::Fail;

use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'||' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorOrFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'||' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorOrSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'^^' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorXorFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'^^' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorXorSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'&&' operator expected boolean as the first operand, but got '{}'",
        _0
    )]
    OperatorAndFirstOperandExpectedBoolean(String),
    #[fail(
        display = "'&&' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorAndSecondOperandExpectedBoolean(String),

    #[fail(
        display = "'==' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedUnit(String),
    #[fail(
        display = "'==' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedBoolean(String),
    #[fail(
        display = "'==' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorEqualsSecondOperandExpectedInteger(String),
    #[fail(
        display = "'==' operator expected a primitive type as the first operand, but got '{}'",
        _0
    )]
    OperatorEqualsFirstOperandExpectedPrimitiveType(String),

    #[fail(
        display = "'!=' operator expected unit as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedUnit(String),
    #[fail(
        display = "'!=' operator expected boolean as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedBoolean(String),
    #[fail(
        display = "'!=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsSecondOperandExpectedInteger(String),
    #[fail(
        display = "'!=' operator expected a primitive type as the first operand, but got '{}'",
        _0
    )]
    OperatorNotEqualsFirstOperandExpectedPrimitiveType(String),

    #[fail(
        display = "'>=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsFirstOperandExpectedInteger(String),
    #[fail(
        display = "'>=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterEqualsSecondOperandExpectedInteger(String),

    #[fail(
        display = "'<=' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsFirstOperandExpectedInteger(String),
    #[fail(
        display = "'<=' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserEqualsSecondOperandExpectedInteger(String),

    #[fail(
        display = "'>' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorGreaterFirstOperandExpectedInteger(String),
    #[fail(
        display = "'>' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorGreaterSecondOperandExpectedInteger(String),

    #[fail(
        display = "'<' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorLesserFirstOperandExpectedInteger(String),
    #[fail(
        display = "'<' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorLesserSecondOperandExpectedInteger(String),

    #[fail(
        display = "'+' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorAdditionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'+' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorAdditionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'-' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorSubtractionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'-' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorSubtractionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'*' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationFirstOperandExpectedInteger(String),
    #[fail(
        display = "'*' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorMultiplicationSecondOperandExpectedInteger(String),

    #[fail(
        display = "'/' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorDivisionFirstOperandExpectedInteger(String),
    #[fail(
        display = "'/' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorDivisionSecondOperandExpectedInteger(String),

    #[fail(
        display = "'%' operator expected integer as the first operand, but got '{}'",
        _0
    )]
    OperatorRemainderFirstOperandExpectedInteger(String),
    #[fail(
        display = "'%' operator expected integer as the second operand, but got '{}'",
        _0
    )]
    OperatorRemainderSecondOperandExpectedInteger(String),

    #[fail(
        display = "unary '-' operator expected an integer as the operand, but got '{}'",
        _0
    )]
    OperatorNegationExpectedInteger(String),
    #[fail(
        display = "'!' operator expected a boolean as the operand, but got '{}'",
        _0
    )]
    OperatorNotExpectedBoolean(String),

    #[fail(
        display = "'[]' operator expected an array as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedArray(String),
    #[fail(
        display = "'[]' operator expected an integer as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedIntegerOrRange(String),
    #[fail(
        display = "'.' operator expected a tuple as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedTuple(String),
    #[fail(
        display = "'.' operator expected a structure as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedStructure(String),

    #[fail(display = "it is impossible to create a value from type {}", _0)]
    ConvertingFromType(String),

    #[fail(display = "{}", _0)]
    Integer(IntegerValueError),
    #[fail(display = "{}", _0)]
    Array(ArrayValueError),
    #[fail(display = "{}", _0)]
    Tuple(TupleValueError),
    #[fail(display = "{}", _0)]
    Structure(StructureValueError),
    #[fail(display = "{}", _0)]
    Casting(CasterError),
}
