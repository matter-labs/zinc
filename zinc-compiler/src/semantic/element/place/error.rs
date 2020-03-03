//!
//! The semantic analyzer place error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorIndexFirstOperandExpectedArray(String),
    OperatorIndexSecondOperandExpectedIntegerOrRange(String),
    OperatorFieldFirstOperandExpectedTuple(String),
    OperatorFieldFirstOperandExpectedStructure(String),

    ArraySliceStartOutOfRange(String),
    ArraySliceEndOutOfRange(String, usize),
    ArraySliceEndLesserThanStart(String, String),
    TupleFieldDoesNotExist(usize, String),
    StructureFieldDoesNotExist(String, String),
}
