//!
//! The semantic analyzer place error.
//!

use zinc_lexical::Location;

///
/// The semantic analyzer place error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Tried to assign an invalid type value to a variable.
    MutatingWithDifferentType {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified expected type.
        expected: String,
        /// The invalid actual type.
        found: String,
    },
    /// Tried to change an immutable variable.
    MutatingImmutableMemory {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The name of the immutable variable.
        name: String,
        /// The location of the immutable variable. `None` for intrinsic items.
        reference: Option<Location>,
    },
    /// Tried to change an immutable contract storage field.
    MutatingImmutableContractField {
        /// The mutating expression location.
        location: Location,
        /// The name of the immutable contract storage field.
        name: String,
    },

    /// The `[]` index operator expects an array value as the first operand.
    OperatorIndexFirstOperandExpectedArray {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `[]` index operator expects an array index integer value or
    /// an array slice range value as the first operand.
    OperatorIndexSecondOperandExpectedIntegerOrRange {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `.` dot access operator expects a tuple value as the first operand.
    OperatorDotFirstOperandExpectedTuple {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `.` dot access operator expects an object instance value as the first operand.
    OperatorDotFirstOperandExpectedInstance {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The slice left bound is negative.
    ArraySliceStartOutOfRange {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The left slice bound as string.
        start: String,
    },
    /// The constant right range bound is out of the compile time-known range.
    ArraySliceEndOutOfRange {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The right slice bound as string.
        end: String,
        /// The actual array size, which is violated by `end`.
        size: usize,
    },
    /// The array slicing range left bound must be not be bigger than the right one.
    ArraySliceEndLesserThanStart {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The left slice bound as string.
        start: String,
        /// The right slice bound as string.
        end: String,
    },
    /// The tuple index cannot be greater or equal to the tuple elements count.
    TupleFieldOutOfRange {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified field type.
        type_identifier: String,
        /// The index that is out of range.
        field_index: usize,
    },
    /// The provided field name does not exist in the structure type.
    StructureFieldDoesNotExist {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified structure type.
        type_identifier: String,
        /// The name of the invalid field.
        field_name: String,
    },
    /// The provided field name does not exist in the contract type.
    ContractFieldDoesNotExist {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified contract type.
        type_identifier: String,
        /// The name of the invalid field.
        field_name: String,
    },
}
