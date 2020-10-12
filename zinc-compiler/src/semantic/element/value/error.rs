//!
//! The semantic analyzer value element error.
//!

use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::contract::error::Error as ContractValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use zinc_lexical::Location;

///
/// The semantic analyzer value element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `||` operator expects a boolean value as the first operand.
    OperatorOrFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `||` operator expects a boolean value as the second operand.
    OperatorOrSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `^^` operator expects a boolean value as the first operand.
    OperatorXorFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `^^` operator expects a boolean value as the second operand.
    OperatorXorSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `&&` operator expects a boolean value as the first operand.
    OperatorAndFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `&&` operator expects a boolean value as the second operand.
    OperatorAndSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `==` operator expects a primitive type value as the first operand.
    /// Primitive types are units, booleans, and integers.
    OperatorEqualsFirstOperandExpectedPrimitiveType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects a unit type value as the second operand.
    OperatorEqualsSecondOperandExpectedUnit {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects a boolean type value as the second operand.
    OperatorEqualsSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects an integer type value as the second operand.
    OperatorEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `!=` operator expects a primitive type value as the first operand.
    /// Primitive types are units, booleans, and integers.
    OperatorNotEqualsFirstOperandExpectedPrimitiveType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects a unit type value as the second operand.
    OperatorNotEqualsSecondOperandExpectedUnit {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects a boolean type value as the second operand.
    OperatorNotEqualsSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects a integer type value as the second operand.
    OperatorNotEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>=` operator expects an integer type value as the first operand.
    OperatorGreaterEqualsFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>=` operator expects an integer type value as the second operand.
    OperatorGreaterEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<=` operator expects an integer type value as the first operand.
    OperatorLesserEqualsFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<=` operator expects an integer type value as the second operand.
    OperatorLesserEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>` operator expects an integer type value as the first operand.
    OperatorGreaterFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>` operator expects an integer type value as the second operand.
    OperatorGreaterSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<` operator expects an integer type value as the first operand.
    OperatorLesserFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<` operator expects an integer type value as the second operand.
    OperatorLesserSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `|` operator expects an integer type value as the first operand.
    OperatorBitwiseOrFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `|` operator expects an integer type value as the second operand.
    OperatorBitwiseOrSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `^` operator expects an integer type value as the first operand.
    OperatorBitwiseXorFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `^` operator expects an integer type value as the second operand.
    OperatorBitwiseXorSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `&` operator expects an integer type value as the first operand.
    OperatorBitwiseAndFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `&` operator expects an integer type value as the second operand.
    OperatorBitwiseAndSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<<` operator expects an integer type value as the first operand.
    OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<<` operator expects an integer type value as the second operand.
    OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>>` operator expects an integer type value as the first operand.
    OperatorBitwiseShiftRightFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>>` operator expects an integer type value as the second operand.
    OperatorBitwiseShiftRightSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `+` operator expects an integer type value as the first operand.
    OperatorAdditionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `+` operator expects an integer type value as the second operand.
    OperatorAdditionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `-` operator expects an integer type value as the first operand.
    OperatorSubtractionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `-` operator expects an integer type value as the second operand.
    OperatorSubtractionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `*` operator expects an integer type value as the first operand.
    OperatorMultiplicationFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `*` operator expects an integer type value as the second operand.
    OperatorMultiplicationSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `/` operator expects an integer type value as the first operand.
    OperatorDivisionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `/` operator expects an integer type value as the second operand.
    OperatorDivisionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `%` operator expects an integer type value as the first operand.
    OperatorRemainderFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `%` operator expects an integer type value as the second operand.
    OperatorRemainderSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `!` operator expects a boolean value as the operand.
    OperatorNotExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `|` operator expects an integer type value as the operand.
    OperatorBitwiseNotExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `-` operator expects an integer type value as the operand.
    OperatorNegationExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `[]` index operator expects an array value as the first operand.
    OperatorIndexFirstOperandExpectedArray {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `[]` index operator expects an array index integer value or
    /// an array slice range value as the first operand.
    OperatorIndexSecondOperandExpectedIntegerOrRange {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `.` dot access operator expects a tuple value as the first operand.
    OperatorDotFirstOperandExpectedTuple {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `.` dot access operator expects an object instance value as the first operand.
    OperatorDotFirstOperandExpectedInstance {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The integer value error. See inner element description.
    Integer(IntegerValueError),
    /// The array value error. See inner element description.
    Array(ArrayValueError),
    /// The tuple value error. See inner element description.
    Tuple(TupleValueError),
    /// The structure value error. See inner element description.
    Structure(StructureValueError),
    /// The contract value error. See inner element description.
    Contract(ContractValueError),
    /// The type caster error. See inner element description.
    Casting {
        /// The error location data.
        location: Location,
        /// The inner type casting error.
        inner: CastingError,
        /// The location of the type casted to.
        reference: Location,
    },
}
