//!
//! The semantic analyzer constant element error.
//!

use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::constant::array::error::Error as ArrayConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::structure::error::Error as StructureConstantError;
use crate::semantic::element::constant::tuple::error::Error as TupleConstantError;
use zinc_lexical::Location;

///
/// The semantic analyzer constant element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `..=` operator expects an integer type constant as the first operand.
    OperatorRangeInclusiveFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `..=` operator expects an integer type constant as the second operand.
    OperatorRangeInclusiveSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `..` operator expects an integer type constant as the first operand.
    OperatorRangeFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `..` operator expects an integer type constant as the second operand.
    OperatorRangeSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `||` operator expects a boolean type constant as the first operand.
    OperatorOrFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `||` operator expects a boolean type constant as the second operand.
    OperatorOrSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `^^` operator expects a boolean type constant as the first operand.
    OperatorXorFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `^^` operator expects a boolean type constant as the second operand.
    OperatorXorSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `&&` operator expects a boolean type constant as the first operand.
    OperatorAndFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `&&` operator expects a boolean type constant as the second operand.
    OperatorAndSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `==` operator expects a primitive type constant as the first operand.
    /// Primitive types are units, booleans, and integers.
    OperatorEqualsFirstOperandExpectedPrimitiveType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects a unit type constant as the second operand.
    OperatorEqualsSecondOperandExpectedUnit {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects a boolean type constant as the second operand.
    OperatorEqualsSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `==` operator expects an integer type constant as the second operand.
    OperatorEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `!=` operator expects a primitive type constant as the first operand.
    /// Primitive types are units, booleans, and integers.
    OperatorNotEqualsFirstOperandExpectedPrimitiveType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects a unit type constant as the second operand.
    OperatorNotEqualsSecondOperandExpectedUnit {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects a boolean type constant as the second operand.
    OperatorNotEqualsSecondOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `!=` operator expects an integer type constant as the second operand.
    OperatorNotEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>=` operator expects an integer type constant as the first operand.
    OperatorGreaterEqualsFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>=` operator expects an integer type constant as the second operand.
    OperatorGreaterEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<=` operator expects an integer type constant as the first operand.
    OperatorLesserEqualsFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<=` operator expects an integer type constant as the second operand.
    OperatorLesserEqualsSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>` operator expects an integer type constant as the first operand.
    OperatorGreaterFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>` operator expects an integer type constant as the second operand.
    OperatorGreaterSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<` operator expects an integer type constant as the first operand.
    OperatorLesserFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<` operator expects an integer type constant as the second operand.
    OperatorLesserSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `|` operator expects an integer type constant as the first operand.
    OperatorBitwiseOrFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `|` operator expects an integer type constant as the second operand.
    OperatorBitwiseOrSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `^` operator expects an integer type constant as the first operand.
    OperatorBitwiseXorFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `^` operator expects an integer type constant as the second operand.
    OperatorBitwiseXorSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `&` operator expects an integer type constant as the first operand.
    OperatorBitwiseAndFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `&` operator expects an integer type constant as the second operand.
    OperatorBitwiseAndSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<<` operator expects an integer type constant as the first operand.
    OperatorBitwiseShiftLeftFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<<` operator expects an integer type constant as the second operand.
    OperatorBitwiseShiftLeftSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>>` operator expects an integer type constant as the first operand.
    OperatorBitwiseShiftRightFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>>` operator expects an integer type constant as the second operand.
    OperatorBitwiseShiftRightSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `+` operator expects an integer type constant as the first operand.
    OperatorAdditionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `+` operator expects an integer type constant as the second operand.
    OperatorAdditionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `-` operator expects an integer type constant as the first operand.
    OperatorSubtractionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `-` operator expects an integer type constant as the second operand.
    OperatorSubtractionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `*` operator expects an integer type constant as the first operand.
    OperatorMultiplicationFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `*` operator expects an integer type constant as the second operand.
    OperatorMultiplicationSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `/` operator expects an integer type constant as the first operand.
    OperatorDivisionFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `/` operator expects an integer type constant as the second operand.
    OperatorDivisionSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `%` operator expects an integer type constant as the first operand.
    OperatorRemainderFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `%` operator expects an integer type constant as the second operand.
    OperatorRemainderSecondOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `!` operator expects a boolean type constant as the operand.
    OperatorNotExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `|` operator expects an integer type constant as the operand.
    OperatorBitwiseNotExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `-` operator expects an integer type constant as the operand.
    OperatorNegationExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `[]` index operator expects an array constant as the first operand.
    OperatorIndexFirstOperandExpectedArray {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `[]` index operator expects an array index integer constant or
    /// an array slice range constant as the first operand.
    OperatorIndexSecondOperandExpectedIntegerOrRange {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `.` dot access operator expects a tuple constant as the first operand.
    OperatorDotFirstOperandExpectedTuple {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `.` dot access operator expects a constant object instance as the first operand.
    OperatorDotFirstOperandExpectedInstance {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The integer constant error. See inner element description.
    Integer(IntegerConstantError),
    /// The array constant error. See inner element description.
    Array(ArrayConstantError),
    /// The tuple constant error. See inner element description.
    Tuple(TupleConstantError),
    /// The structure constant error. See inner element description.
    Structure(StructureConstantError),
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
