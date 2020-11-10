//!
//! The semantic analyzer error.
//!

use zinc_lexical::Location;

use num::BigInt;

use crate::semantic::casting::error::Error as CastingError;

///
/// The semantic analyzer error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The integer constant processing error.
    InvalidInteger {
        /// The error location data.
        location: Location,
        /// The inner inference error.
        inner: zinc_math::Error,
    },

    /// The `=` operator expects a memory place as the first operand.
    OperatorAssignmentFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `=` operator expects an evaluable element as the second operand.
    OperatorAssignmentSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `|=` operator expects a memory place as the first operand.
    OperatorAssignmentBitwiseOrFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `|=` operator expects an evaluable element as the second operand.
    OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `^=` operator expects a memory place as the first operand.
    OperatorAssignmentBitwiseXorFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `^=` operator expects an evaluable element as the second operand.
    OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `&=` operator expects a memory place as the first operand.
    OperatorAssignmentBitwiseAndFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `&=` operator expects an evaluable element as the second operand.
    OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `<<=` operator expects a memory place as the first operand.
    OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `<<=` operator expects an evaluable element as the second operand.
    OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `>>=` operator expects a memory place as the first operand.
    OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `>>=` operator expects an evaluable element as the second operand.
    OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `+=` operator expects a memory place as the first operand.
    OperatorAssignmentAdditionFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `+=` operator expects an evaluable element as the second operand.
    OperatorAssignmentAdditionSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `-=` operator expects a memory place as the first operand.
    OperatorAssignmentSubtractionFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `-=` operator expects an evaluable element as the second operand.
    OperatorAssignmentSubtractionSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `*=` operator expects a memory place as the first operand.
    OperatorAssignmentMultiplicationFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `*=` operator expects an evaluable element as the second operand.
    OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `/=` operator expects a memory place as the first operand.
    OperatorAssignmentDivisionFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `/=` operator expects an evaluable element as the second operand.
    OperatorAssignmentDivisionSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `%=` operator expects a memory place as the first operand.
    OperatorAssignmentRemainderFirstOperandExpectedPlace {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `%=` operator expects an evaluable element as the second operand.
    OperatorAssignmentRemainderSecondOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The `..` operator expects a constant element as the first operand.
    OperatorRangeFirstOperandExpectedConstant {
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
    /// The `..` operator expects a constant element as the second operand.
    OperatorRangeSecondOperandExpectedConstant {
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

    /// The `..=` operator expects a constant element as the first operand.
    OperatorRangeInclusiveFirstOperandExpectedConstant {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `..=` operator expects an integer type constant as the first operand.
    OperatorRangeInclusiveFirstOperandExpectedInteger {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `..=` operator expects a constant element as the second operand.
    OperatorRangeInclusiveSecondOperandExpectedConstant {
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

    /// The `||` operator expects an evaluable element as the first operand.
    OperatorOrFirstOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `||` operator expects a boolean value as the first operand.
    OperatorOrFirstOperandExpectedBoolean {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `||` operator expects an evaluable element as the second operand.
    OperatorOrSecondOperandExpectedEvaluable {
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

    /// The `^^` operator expects an evaluable element as the first operand.
    OperatorXorFirstOperandExpectedEvaluable {
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
    /// The `^^` operator expects an evaluable element as the second operand.
    OperatorXorSecondOperandExpectedEvaluable {
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

    /// The `&&` operator expects an evaluable element as the first operand.
    OperatorAndFirstOperandExpectedEvaluable {
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
    /// The `&&` operator expects an evaluable element as the second operand.
    OperatorAndSecondOperandExpectedEvaluable {
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

    /// The `==` operator expects an evaluable element as the first operand.
    OperatorEqualsFirstOperandExpectedEvaluable {
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
    /// The `==` operator expects an evaluable element as the second operand.
    OperatorEqualsSecondOperandExpectedEvaluable {
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
    /// The `==` operator expects two integer values of the same type.
    OperatorEqualsTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `!=` operator expects an evaluable element as the first operand.
    OperatorNotEqualsFirstOperandExpectedEvaluable {
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
    /// The `!=` operator expects an evaluable element as the second operand.
    OperatorNotEqualsSecondOperandExpectedEvaluable {
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
    /// The `!=` operator expects two integer values of the same type.
    OperatorNotEqualsTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `>=` operator expects an evaluable element as the first operand.
    OperatorGreaterEqualsFirstOperandExpectedEvaluable {
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
    /// The `>=` operator expects an evaluable element as the second operand.
    OperatorGreaterEqualsSecondOperandExpectedEvaluable {
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
    /// The `>=` operator expects two integer values of the same type.
    OperatorGreaterEqualsTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `<=` operator expects an evaluable element as the first operand.
    OperatorLesserEqualsFirstOperandExpectedEvaluable {
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
    /// The `<=` operator expects an evaluable element as the second operand.
    OperatorLesserEqualsSecondOperandExpectedEvaluable {
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
    /// The `<=` operator expects two integer values of the same type.
    OperatorLesserEqualsTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `>` operator expects an evaluable element as the first operand.
    OperatorGreaterFirstOperandExpectedEvaluable {
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
    /// The `>` operator expects an evaluable element as the second operand.
    OperatorGreaterSecondOperandExpectedEvaluable {
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
    /// The `>` operator expects two integer values of the same type.
    OperatorGreaterTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `<` operator expects an evaluable element as the first operand.
    OperatorLesserFirstOperandExpectedEvaluable {
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
    /// The `<` operator expects an evaluable element as the second operand.
    OperatorLesserSecondOperandExpectedEvaluable {
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
    /// The `<` operator expects two integer values of the same type.
    OperatorLesserTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `|` operator expects an evaluable element as the first operand.
    OperatorBitwiseOrFirstOperandExpectedEvaluable {
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
    /// The `|` operator expects an evaluable element as the second operand.
    OperatorBitwiseOrSecondOperandExpectedEvaluable {
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
    /// The `|` operator expects two integer values of the same type.
    OperatorBitwiseOrTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `^` operator expects an evaluable element as the first operand.
    OperatorBitwiseXorFirstOperandExpectedEvaluable {
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
    /// The `^` operator expects an evaluable element as the second operand.
    OperatorBitwiseXorSecondOperandExpectedEvaluable {
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
    /// The `^` operator expects two integer values of the same type.
    OperatorBitwiseXorTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `&` operator expects an evaluable element as the first operand.
    OperatorBitwiseAndFirstOperandExpectedEvaluable {
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
    /// The `&` operator expects an evaluable element as the second operand.
    OperatorBitwiseAndSecondOperandExpectedEvaluable {
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
    /// The `&` operator expects two integer values of the same type.
    OperatorBitwiseAndTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },

    /// The `<<` operator expects an evaluable element as the first operand.
    OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
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
    /// The `<<` operator expects an evaluable element as the second operand.
    OperatorBitwiseShiftLeftSecondOperandExpectedConstant {
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
    /// The `<<` operator expects an unsigned integer as the second operand.
    OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned {
        /// The error location data.
        location: Location,
        /// The stringified second operand.
        found: String,
    },

    /// The `>>` operator expects an evaluable element as the first operand.
    OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
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
    /// The `>>` operator expects an evaluable element as the second operand.
    OperatorBitwiseShiftRightSecondOperandExpectedConstant {
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
    /// The `>>` operator expects an unsigned integer as the second operand.
    OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned {
        /// The error location data.
        location: Location,
        /// The stringified second operand.
        found: String,
    },

    /// The bitwise operators are forbidden for the signed types.
    OperatorBitwiseSignedOperandForbidden {
        /// The error location data.
        location: Location,
    },
    /// The bitwise operators are forbidden for the `field` type.
    OperatorBitwiseFieldOperandForbidden {
        /// The error location data.
        location: Location,
    },

    /// The `+` operator expects an evaluable element as the first operand.
    OperatorAdditionFirstOperandExpectedEvaluable {
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
    /// The `+` operator expects an evaluable element as the second operand.
    OperatorAdditionSecondOperandExpectedEvaluable {
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
    /// The `+` operator expects two integer values of the same type.
    OperatorAdditionTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The binary `+` operator overflow.
    OperatorAdditionOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },

    /// The `-` operator expects an evaluable element as the first operand.
    OperatorSubtractionFirstOperandExpectedEvaluable {
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
    /// The `-` operator expects an evaluable element as the second operand.
    OperatorSubtractionSecondOperandExpectedEvaluable {
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
    /// The `-` operator expects two integer values of the same type.
    OperatorSubtractionTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The binary `-` operator overflow.
    OperatorSubtractionOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },

    /// The `*` operator expects an evaluable element as the first operand.
    OperatorMultiplicationFirstOperandExpectedEvaluable {
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
    /// The `*` operator expects an evaluable element as the second operand.
    OperatorMultiplicationSecondOperandExpectedEvaluable {
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
    /// The `*` operator expects two integer values of the same type.
    OperatorMultiplicationTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The binary `*` operator overflow.
    OperatorMultiplicationOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },

    /// The `/` operator expects an evaluable element as the first operand.
    OperatorDivisionFirstOperandExpectedEvaluable {
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
    /// The `/` operator expects an evaluable element as the second operand.
    OperatorDivisionSecondOperandExpectedEvaluable {
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
    /// The `/` operator expects two integer values of the same type.
    OperatorDivisionTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The binary `/` operator overflow.
    OperatorDivisionOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The division `/` operator is forbidden for the `field` type.
    OperatorDivisionFieldOperandForbidden {
        /// The error location data.
        location: Location,
    },
    /// Division by zero.
    OperatorDivisionByZero {
        /// The error location data.
        location: Location,
    },

    /// The `%` operator expects an evaluable element as the first operand.
    OperatorRemainderFirstOperandExpectedEvaluable {
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
    /// The `%` operator expects an evaluable element as the second operand.
    OperatorRemainderSecondOperandExpectedEvaluable {
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
    /// The `%` operator expects two integer values of the same type.
    OperatorRemainderTypesMismatch {
        /// The error location data.
        location: Location,
        /// The stringified first operand.
        first: String,
        /// The stringified second operand.
        second: String,
    },
    /// The binary `%` operator overflow.
    OperatorRemainderOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The remainder `%` operator is forbidden for the `field` type.
    OperatorRemainderFieldOperandForbidden {
        /// The error location data.
        location: Location,
    },
    /// Remainder through division by zero.
    OperatorRemainderOfDivisionByZero {
        /// The error location data.
        location: Location,
    },

    /// The `as` operator expects an evaluable element as the first operand.
    OperatorCastingFirstOperandExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `as` operator expects a type element as the second operand.
    OperatorCastingSecondOperandExpectedType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `as` operator got types that cannot be casted.
    OperatorCastingTypesMismatch {
        /// The error location data.
        location: Location,
        /// The inner type casting error.
        inner: CastingError,
        /// The location of the type casted to.
        reference: Location,
    },
    /// The binary `as` operator overflow.
    OperatorCastingOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },

    /// The unary `!` operator expects an evaluable element as the operand.
    OperatorNotExpectedEvaluable {
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

    /// The unary `|` operator expects an evaluable element as the operand.
    OperatorBitwiseNotExpectedEvaluable {
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

    /// The unary `-` operator expects an evaluable element as the operand.
    OperatorNegationExpectedEvaluable {
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
    /// The unary `-` operator overflow.
    OperatorNegationOverflow {
        /// The error location data.
        location: Location,
        /// The value which overflowes `r#type`.
        value: BigInt,
        /// The type overflowed by `value`.
        r#type: String,
    },
    /// The negation `-` operator is forbidden for the `field` type.
    OperatorNegationFieldOperandForbidden {
        /// The error location data.
        location: Location,
    },

    /// The index `[]` operator expects a memory place or evaluable element as the first operand.
    OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The `[]` index operator expects an array value as the first operand.
    OperatorIndexFirstOperandExpectedArray {
        /// The memory descriptor location, usually a variable name.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The index `[]` operator expects an evaluable element as the second operand.
    OperatorIndexSecondOperandExpectedEvaluable {
        /// The error location data.
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

    /// The dot `.` operator expects a memory place or evaluable element as the first operand.
    OperatorDotFirstOperandExpectedPlaceOrEvaluable {
        /// The error location data.
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
    /// The dot `.` operator expects an evaluable element as the second operand.
    OperatorDotSecondOperandExpectedIdentifier {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The path `::` operator expects a path element as the first operand.
    OperatorPathFirstOperandExpectedPath {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The path `::` operator expects an identifier as the second operand.
    OperatorPathSecondOperandExpectedIdentifier {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The structure `{ ... }` operator expects a type as the first operand.
    OperatorStructureFirstOperandExpectedType {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },
    /// The structure `{ ... }` operator expects a structure literal as the second operand.
    OperatorStructureSecondOperandExpectedLiteral {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The subsequent element type is not equal to the first element type, which dictates the array element type.
    ArrayPushingInvalidType {
        /// The error location data.
        location: Location,
        /// The expected array type, which is dictated by the first element pushed.
        expected: String,
        /// The invalid array element, which is actually found.
        found: String,
    },
    /// The constant array index is out of the compile time-known range.
    ArrayIndexOutOfRange {
        /// The error location data.
        location: Location,
        /// The invalid array index, which is actually found.
        index: String,
        /// The actual array size, which is violated by `index`.
        size: usize,
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
        r#type: String,
        /// The index that is out of range.
        field_index: usize,
    },

    /// The structure type appeared in the code without the structure literal with fields.
    StructureNotInitialized {
        /// The error location data.
        location: Location,
        /// The stringified uninitiliazed structure type.
        r#type: String,
    },
    /// The provided field name does not exist in the structure type.
    StructureFieldDoesNotExist {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        r#type: String,
        /// The name of the invalid field.
        field_name: String,
    },
    /// A provided field name does not match the one in the structure type at the same position.
    StructureFieldExpected {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        r#type: String,
        /// The position of the erroneous field.
        position: usize,
        /// The name of the expected field.
        expected: String,
        /// The name of the invalid field, which was actually found.
        found: String,
    },
    /// A provided field type does not match the one in the structure type.
    StructureFieldInvalidType {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        r#type: String,
        /// The erroneous field name.
        field_name: String,
        /// The expected type for the field.
        expected: String,
        /// The invalid type, which was actually found.
        found: String,
    },
    /// The number of provided fields is bigger than the expected one.
    StructureFieldOutOfRange {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        r#type: String,
        /// The expected number of structure fields.
        expected: usize,
        /// The position of the provided structure field.
        found: usize,
    },

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

    /// The path expression must point to a type, but points to another kind of item.
    TypeAliasExpectedType {
        /// The path expression location.
        location: Location,
        /// The stringified item, found instead of a type.
        found: String,
    },
    /// Some built-in types cannot be instantiated.
    TypeInstantiationForbidden {
        /// The error location.
        location: Location,
        /// The found type.
        found: String,
    },
    /// A field with the same name occurs for the second time.
    TypeDuplicateField {
        /// The duplicate field location.
        location: Location,
        /// The structure type name.
        r#type: String,
        /// The duplicate field name.
        field_name: String,
    },
    /// A variant with the same value occurs for the second time.
    TypeDuplicateVariantValue {
        /// The duplicate variant location.
        location: Location,
        /// The enumeration type name.
        r#type: String,
        /// The duplicate variant name.
        variant_name: String,
        /// The duplicate variant value.
        variant_value: BigInt,
    },
    /// The generic items are not supported by the type.
    TypeUnexpectedGenerics {
        /// The path expression location.
        location: Location,
        /// The type which does not expect generics.
        r#type: String,
    },
    /// The type expected generics, but got none.
    TypeExpectedGenerics {
        /// The type initializer location.
        location: Location,
        /// The type name.
        r#type: String,
        /// The number of expected generics.
        expected: usize,
    },
    /// The structure expected different number of generic arguments.
    TypeInvalidGenericsNumber {
        /// The type initializer location.
        location: Location,
        /// The structure type name.
        r#type: String,
        /// The number of expected generics.
        expected: usize,
        /// The number of found generics.
        found: usize,
    },

    /// The actual arguments number does not match the formal arguments number.
    FunctionArgumentCount {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The expected number of arguments.
        expected: usize,
        /// The actual number of arguments.
        found: usize,
        /// The reference to the function argument list location in the function prototype.
        reference: Option<Location>,
    },
    /// The actual arguments number does not match the formal arguments number.
    FunctionDebugArgumentCount {
        /// The error location data.
        location: Location,
        /// The expected number of arguments.
        expected: usize,
        /// The actual number of arguments.
        found: usize,
    },
    /// The argument type does not match the expected one.
    FunctionArgumentType {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The name of the argument, whose actual type is invalid.
        name: String,
        /// The position of the invalid argument.
        position: usize,
        /// The stringified expected type.
        expected: String,
        /// The actual invalid type.
        found: String,
    },
    /// The argument must be a constant, but it is not.
    FunctionArgumentConstantness {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The name of the non-constant argument.
        name: String,
        /// The position of the non-constant argument.
        position: usize,
        /// The stringified non-constant argument.
        found: String,
    },
    /// The argument cannot be treated as value.
    FunctionArgumentNotEvaluable {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The position of the invalid argument.
        position: usize,
        /// The stringified invalid argument.
        found: String,
    },
    /// The function returns a value, whose type does not match the one in the function prototype.
    FunctionReturnType {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The stringified type, which is expected.
        expected: String,
        /// The stringified invalid type.
        found: String,
        /// The reference to the return type location in the function prototype.
        reference: Location,
    },
    /// Calling a non-callable object, that is, not a function.
    FunctionNonCallable {
        /// The error location data.
        location: Location,
        /// The non-callable item identifier.
        name: String,
    },
    /// A mutable method was called with an immutable instance.
    FunctionCallMutableFromImmutable {
        /// The function location.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// Tried to call a function with the `!` specifier, but the function does not require it.
    FunctionUnexpectedExclamationMark {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// Some intrinsic functions can only be called with the `!` specifier.
    FunctionExpectedExclamationMark {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: &'static str,
    },
    /// An array is tried to be truncated to a bigger size. The `pad` function must be used instead.
    FunctionStdlibArrayTruncatingToBiggerSize {
        /// The error location data.
        location: Location,
        /// The original lesser array size.
        from: usize,
        /// The new invalid bigger array size.
        to: usize,
    },
    /// An array is tried to be padded to a lesser size. The `truncate` function must be used instead.
    FunctionStdlibArrayPaddingToLesserSize {
        /// The error location data.
        location: Location,
        /// The original bigger array size.
        from: usize,
        /// The new invalid lesser array size.
        to: usize,
    },
    /// The new length value cannot be converted to `usize` type.
    FunctionStdlibArrayNewLengthInvalid {
        /// The error location data.
        location: Location,
        /// The stringified new length argument value.
        value: String,
    },

    /// The unit test function cannot be called.
    UnitTestCallForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function must be only declared at the module root.
    UnitTestBeyondModuleScope {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot be public.
    UnitTestPublicForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot be constant.
    UnitTestConstantForbidden {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot have arguments.
    UnitTestCannotHaveArguments {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The unit test function cannot return a value.
    UnitTestCannotReturnValue {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },

    /// The item is undeclared within the current scope stack.
    ScopeItemUndeclared {
        /// The error location data.
        location: Location,
        /// The undeclared item name.
        name: String,
    },
    /// The item is already declared within the current scope stack.
    ScopeItemRedeclared {
        /// The error location data.
        location: Location,
        /// The redeclared item name.
        name: String,
        /// The location where the item is declared for the first item. `None` for intrinsic items.
        reference: Option<Location>,
    },
    /// The item is not a namespace, and cannot be a part of a path expression.
    ScopeExpectedNamespace {
        /// The error location data.
        location: Location,
        /// The non-namespace item name.
        name: String,
    },
    /// Another contract is already declared within the scope stack.
    /// Only one contract is allowed per application.
    ScopeContractRedeclared {
        /// The error location data.
        location: Location,
        /// The location where the first contract is declared.
        reference: Location,
    },
    /// There is a reference loop between items. That is, there are some items referencing each
    /// other. Perhaps, not directly, that is, through one or more other items.
    ScopeReferenceLoop {
        /// The error location data.
        location: Location,
    },

    /// A non-constant element is found in a constant context.
    ExpressionNonConstantElement {
        /// The error location data.
        location: Location,
        /// The strigified invalid element.
        found: String,
    },

    /// The condition is not of boolean type.
    ConditionalExpectedBooleanCondition {
        /// The error location data.
        location: Location,
        /// The invalid condition type, which is actually found.
        found: String,
    },
    /// The conditional branches must return the same type, but it is not so.
    ConditionalBranchTypesMismatch {
        /// The error location data.
        location: Location,
        /// The expected type, which is dictated by the first (main- or then-) branch result.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The another branch location, which helps user to find the error.
        reference: Location,
    },

    /// Only primitive types can act as scrutinee types (be matched) for now.
    MatchScrutineeInvalidType {
        /// The error location data.
        location: Location,
        /// The invalid type, which is actually found.
        found: String,
    },
    /// The `match` patterns do not cover all the possible values of the scrutinee expression type.
    MatchNotExhausted {
        /// The error location data.
        location: Location,
    },
    /// A `match` expression must have at least two branches to generate a useful conditional code.
    MatchLessThanTwoBranches {
        /// The error location data.
        location: Location,
    },
    /// A branch with an refutable pattern appears after the irrefutable one, that is, after the
    /// branch, whose pattern always matches.
    MatchBranchUnreachable {
        /// The error location data.
        location: Location,
    },
    /// Only constants can act as the branch patterns.
    MatchBranchPatternPathExpectedConstant {
        /// The error location data.
        location: Location,
        /// The invalid expression, which is actually found.
        found: String,
    },
    /// A branch pattern type does not match the scrutinee expression type.
    MatchBranchPatternInvalidType {
        /// The error location data.
        location: Location,
        /// The expected branch pattern type, which is dictated by the scrutinee expression type.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The another branch location, which helps user to find the error.
        reference: Location,
    },
    /// A subsequent branch result expression type does not match the first branch expression type.
    MatchBranchExpressionInvalidType {
        /// The error location data.
        location: Location,
        /// The expected branch result type, which is dictated by the first branch result type.
        expected: String,
        /// The invalid type, which is actually found.
        found: String,
        /// The first branch location, which helps user to find the error.
        reference: Location,
    },
    /// Some branch pattern occurs more than once in the `match` expression.
    MatchBranchDuplicate {
        /// The error location data.
        location: Location,
        /// The first branch location, which helps user to find the error.
        reference: Location,
    },

    /// The `while` condition is not of boolean type.
    ForStatementWhileExpectedBooleanCondition {
        /// The condition expression location.
        location: Location,
        /// The stringified invalid condition type.
        found: String,
    },
    /// The loop bounds is not a constant range expression.
    ForStatementBoundsExpectedConstantRangeExpression {
        /// The loop bounds expression location.
        location: Location,
        /// The stringified invalid bounds element.
        found: String,
    },

    /// Only structure or enumeration types can have an implementation, but another type was found.
    ImplStatementExpectedStructureOrEnumeration {
        /// The invalid type location in the code.
        location: Location,
        /// The invalid type identifier.
        found: String,
    },

    /// The element after the `use` keyword must be a path to an item.
    UseStatementExpectedPath {
        /// The invalid element location in the code.
        location: Location,
        /// The stringified invalid element.
        found: String,
    },

    /// The attribute is unknown. Check the known attribute list for more information.
    UnknownAttribute {
        /// The error location data.
        location: Location,
        /// The invalid stringified attribute.
        found: String,
    },

    /// The type must be explicitly specified for this binding.
    BindingTypeRequired {
        /// The binding location.
        location: Location,
        /// The binding identifier.
        identifier: String,
    },
    /// The binding list expects a tuple expression.
    BindingExpectedTuple {
        /// The invalid pattern location.
        location: Location,
        /// The expected tuple length.
        expected: usize,
        /// The found type.
        found: String,
    },
    /// The `self` argument of a method must be first in argument list, but it is not.
    BindingSelfNotFirstMethodArgument {
        /// The invalid argument location.
        location: Location,
        /// The binding identifier.
        name: String,
        /// The invalid argument position.
        position: usize,
    },
    /// Tuple function argument destructuring is not implemented yet.
    BindingFunctionArgumentDestructuringUnavailable {
        /// The error location.
        location: Location,
    },

    /// The application does not have an entry point function.
    EntryPointMissing,
    /// The application has both the `main` function and contract.
    EntryPointAmbiguous {
        /// The location where the `main` function is declared.
        main: Location,
        /// The location where the contract is declared.
        contract: Location,
    },
    /// The application entry function cannot be constant.
    EntryPointConstant {
        /// The location where the constant `main` function is declared.
        location: Location,
    },
    /// The application entry `main` function is declared outside the application entry module.
    FunctionMainBeyondEntry {
        /// The location where the `main` function is declared.
        location: Location,
    },
    /// The application contract is declared outside the application entry module.
    ContractBeyondEntry {
        /// The location where the contract is declared.
        location: Location,
    },
    /// The source code file for module `name` cannot be found.
    ModuleFileNotFound {
        /// The location where the module is declared.
        location: Location,
        /// The module name, source code for which is absent.
        name: String,
    },
}
