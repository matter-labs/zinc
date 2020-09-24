//!
//! The semantic analyzer constant integer element error.
//!

use num_bigint::BigInt;

use zinc_utils::InferenceError;

use crate::lexical::token::location::Location;

///
/// The semantic analyzer constant integer element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `==` operator expects two integer values of the same type.
    TypesMismatchEquals {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `!=` operator expects two integer values of the same type.
    TypesMismatchNotEquals {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `>=` operator expects two integer values of the same type.
    TypesMismatchGreaterEquals {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `<=` operator expects two integer values of the same type.
    TypesMismatchLesserEquals {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `>` operator expects two integer values of the same type.
    TypesMismatchGreater {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `<` operator expects two integer values of the same type.
    TypesMismatchLesser {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `|` operator expects two integer values of the same type.
    TypesMismatchBitwiseOr {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `^` operator expects two integer values of the same type.
    TypesMismatchBitwiseXor {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `&` operator expects two integer values of the same type.
    TypesMismatchBitwiseAnd {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `+` operator expects two integer values of the same type.
    TypesMismatchAddition {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `-` operator expects two integer values of the same type.
    TypesMismatchSubtraction {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `*` operator expects two integer values of the same type.
    TypesMismatchMultiplication {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `/` operator expects two integer values of the same type.
    TypesMismatchDivision {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The `%` operator expects two integer values of the same type.
    TypesMismatchRemainder {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `<<` operator expects an unsigned integer as the second operand.
    OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
        /// The error location data.
        location: Location,
        /// The stringified second operand.
        found: String,
    },
    /// The `>>` operator expects an unsigned integer as the second operand.
    OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
        /// The error location data.
        location: Location,
        /// The stringified second operand.
        found: String,
    },

    /// The binary `+` operator overflow.
    OverflowAddition {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The binary `-` operator overflow.
    OverflowSubtraction {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The binary `*` operator overflow.
    OverflowMultiplication {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The binary `/` operator overflow.
    OverflowDivision {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The binary `%` operator overflow.
    OverflowRemainder {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The binary `as` operator overflow.
    OverflowCasting {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The unary `-` operator overflow.
    OverflowNegation {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },

    /// The division `/` operator is forbidden for the `field` type.
    ForbiddenFieldDivision {
        /// The error location data.
        location: Location,
    },
    /// The remainder `%` operator is forbidden for the `field` type.
    ForbiddenFieldRemainder {
        /// The error location data.
        location: Location,
    },
    /// The bitwise operators are forbidden for the signed types.
    ForbiddenSignedBitwise {
        /// The error location data.
        location: Location,
    },
    /// The bitwise operators are forbidden for the `field` type.
    ForbiddenFieldBitwise {
        /// The error location data.
        location: Location,
    },
    /// The negation `-` operator is forbidden for the `field` type.
    ForbiddenFieldNegation {
        /// The error location data.
        location: Location,
    },

    /// Division by zero.
    ZeroDivision {
        /// The error location data.
        location: Location,
    },
    /// Remainder through division by zero.
    ZeroRemainder {
        /// The error location data.
        location: Location,
    },

    /// The integer constant is too large.
    IntegerTooLarge {
        /// The error location data.
        location: Location,
        /// The inner inference error.
        inner: InferenceError,
    },
    /// The integer exponent is too small.
    ExponentTooSmall {
        /// The error location data.
        location: Location,
    },
    /// The integer exponent is too large.
    ExponentTooLarge {
        /// The error location data.
        location: Location,
    },
}
