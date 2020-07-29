//!
//! The semantic analyzer element error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::value::error::Error as ValueError;

///
/// The semantic analyzer element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
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

    /// The `..=` operator expects a constant element as the first operand.
    OperatorRangeInclusiveFirstOperandExpectedConstant {
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
    /// The `..` operator expects a constant element as the first operand.
    OperatorRangeFirstOperandExpectedConstant {
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

    /// The `||` operator expects an evaluable element as the first operand.
    OperatorOrFirstOperandExpectedEvaluable {
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

    /// The `^^` operator expects an evaluable element as the first operand.
    OperatorXorFirstOperandExpectedEvaluable {
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

    /// The `&&` operator expects an evaluable element as the first operand.
    OperatorAndFirstOperandExpectedEvaluable {
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

    /// The `==` operator expects an evaluable element as the first operand.
    OperatorEqualsFirstOperandExpectedEvaluable {
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

    /// The `!=` operator expects an evaluable element as the first operand.
    OperatorNotEqualsFirstOperandExpectedEvaluable {
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

    /// The `>=` operator expects an evaluable element as the first operand.
    OperatorGreaterEqualsFirstOperandExpectedEvaluable {
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

    /// The `<=` operator expects an evaluable element as the first operand.
    OperatorLesserEqualsFirstOperandExpectedEvaluable {
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

    /// The `>` operator expects an evaluable element as the first operand.
    OperatorGreaterFirstOperandExpectedEvaluable {
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

    /// The `<` operator expects an evaluable element as the first operand.
    OperatorLesserFirstOperandExpectedEvaluable {
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

    /// The `|` operator expects an evaluable element as the first operand.
    OperatorBitwiseOrFirstOperandExpectedEvaluable {
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

    /// The `^` operator expects an evaluable element as the first operand.
    OperatorBitwiseXorFirstOperandExpectedEvaluable {
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

    /// The `&` operator expects an evaluable element as the first operand.
    OperatorBitwiseAndFirstOperandExpectedEvaluable {
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

    /// The `<<` operator expects an evaluable element as the first operand.
    OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable {
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

    /// The `>>` operator expects an evaluable element as the first operand.
    OperatorBitwiseShiftRightFirstOperandExpectedEvaluable {
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

    /// The `+` operator expects an evaluable element as the first operand.
    OperatorAdditionFirstOperandExpectedEvaluable {
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

    /// The `-` operator expects an evaluable element as the first operand.
    OperatorSubtractionFirstOperandExpectedEvaluable {
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

    /// The `*` operator expects an evaluable element as the first operand.
    OperatorMultiplicationFirstOperandExpectedEvaluable {
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

    /// The `/` operator expects an evaluable element as the first operand.
    OperatorDivisionFirstOperandExpectedEvaluable {
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

    /// The `%` operator expects an evaluable element as the first operand.
    OperatorRemainderFirstOperandExpectedEvaluable {
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

    /// The unary `!` operator expects an evaluable element as the operand.
    OperatorNotExpectedEvaluable {
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

    /// The unary `-` operator expects an evaluable element as the operand.
    OperatorNegationExpectedEvaluable {
        /// The error location data.
        location: Location,
        /// The stringified invalid element found instead.
        found: String,
    },

    /// The index `[]` operator expects a memory place or evaluable element as the first operand.
    OperatorIndexFirstOperandExpectedPlaceOrEvaluable {
        /// The error location data.
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

    /// The dot `.` operator expects a memory place or evaluable element as the first operand.
    OperatorDotFirstOperandExpectedPlaceOrEvaluable {
        /// The error location data.
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

    /// The memory place error. See inner element description.
    Place(PlaceError),
    /// The runtime value error. See inner element description.
    Value(ValueError),
    /// The constant value error. See inner element description.
    Constant(ConstantError),
    /// The type error. See inner element description.
    Type(TypeError),
}
