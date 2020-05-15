//!
//! The semantic analyzer place error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    MutatingWithDifferentType {
        location: Location,
        expected: String,
        found: String,
    },
    MutatingImmutableMemory {
        location: Location,
        name: String,
        reference: Option<Location>,
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

    ArraySliceStartOutOfRange {
        location: Location,
        start: String,
    },
    ArraySliceEndOutOfRange {
        location: Location,
        end: String,
        size: usize,
    },
    ArraySliceEndLesserThanStart {
        location: Location,
        start: String,
        end: String,
    },
    TupleFieldDoesNotExist {
        location: Location,
        type_identifier: String,
        field_index: usize,
    },
    StructureFieldDoesNotExist {
        location: Location,
        type_identifier: String,
        field_name: String,
    },
    ContractFieldDoesNotExist {
        location: Location,
        type_identifier: String,
        field_name: String,
    },
}
