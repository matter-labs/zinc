//!
//! The semantic analyzer place error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    OperatorIndexFirstOperandExpectedArray {
        found: String,
    },
    OperatorIndexSecondOperandExpectedIntegerOrRange {
        found: String,
    },
    OperatorFieldFirstOperandExpectedTuple {
        found: String,
    },
    OperatorFieldFirstOperandExpectedStructure {
        found: String,
    },

    ArraySliceStartOutOfRange {
        start: String,
    },
    ArraySliceEndOutOfRange {
        end: String,
        size: usize,
    },
    ArraySliceEndLesserThanStart {
        start: String,
        end: String,
    },
    TupleFieldDoesNotExist {
        type_identifier: String,
        field_index: usize,
    },
    StructureFieldDoesNotExist {
        type_identifier: String,
        field_name: String,
    },
}
