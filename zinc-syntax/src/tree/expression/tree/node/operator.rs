//!
//! The expression operator.
//!

use std::fmt;

///
/// An expression tree operator.
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    /// The `=` operator.
    Assignment,
    /// The `|=` operator.
    AssignmentBitwiseOr,
    /// The `^=` operator.
    AssignmentBitwiseXor,
    /// The `&=` operator.
    AssignmentBitwiseAnd,
    /// The `<<=` operator.
    AssignmentBitwiseShiftLeft,
    /// The `>>=` operator.
    AssignmentBitwiseShiftRight,
    /// The `+=` operator.
    AssignmentAddition,
    /// The `-=` operator.
    AssignmentSubtraction,
    /// The `*=` operator.
    AssignmentMultiplication,
    /// The `/=` operator.
    AssignmentDivision,
    /// The `%=` operator.
    AssignmentRemainder,

    /// The `..` operator.
    Range,
    /// The `..=` operator.
    RangeInclusive,

    /// The `||` operator.
    Or,
    /// The `^^` operator.
    Xor,
    /// The `&&` operator.
    And,

    /// The `==` operator.
    Equals,
    /// The `!=` operator.
    NotEquals,
    /// The `>=` operator.
    GreaterEquals,
    /// The `<=` operator.
    LesserEquals,
    /// The `>` operator.
    Greater,
    /// The `<` operator.
    Lesser,

    /// The `|` operator.
    BitwiseOr,
    /// The `^` operator.
    BitwiseXor,
    /// The `&` operator.
    BitwiseAnd,
    /// The `<<` operator.
    BitwiseShiftLeft,
    /// The `>>` operator.
    BitwiseShiftRight,

    /// The `+` operator.
    Addition,
    /// The binary `-` operator.
    Subtraction,
    /// The `*` operator.
    Multiplication,
    /// The `/` operator.
    Division,
    /// The `%` operator.
    Remainder,

    /// The `as` operator.
    Casting,

    /// The `!` operator.
    Not,

    /// The `~` operator.
    BitwiseNot,

    /// The unary `-` operator.
    Negation,

    /// The `[]` operator.
    Index,
    /// The `.` operator.
    Dot,

    /// The intrinsic function call `!` quasi-operator.
    CallIntrinsic,
    /// The function call `( ... )` quasi-operator.
    Call,

    /// The `::` operator.
    Path,

    /// The structure literal `{ ... }` quasi-operator.
    Structure,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path => write!(f, "::"),
            _ => todo!(),
        }
    }
}
