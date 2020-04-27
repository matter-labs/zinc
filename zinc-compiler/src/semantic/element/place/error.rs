//!
//! The semantic analyzer place error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    MutatingWithDifferentType {
        expected: String,
        found: String,
    },
    MutatingImmutableMemory {
        name: String,
        reference: Option<Location>,
    },

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
    ContractFieldDoesNotExist {
        type_identifier: String,
        field_name: String,
    },
}
