//!
//! The expression operator.
//!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // assignment
    Assignment,
    AssignmentAddition,
    AssignmentSubtraction,
    AssignmentMultiplication,
    AssignmentDivision,
    AssignmentRemainder,
    AssignmentBitwiseOr,
    AssignmentBitwiseXor,
    AssignmentBitwiseAnd,
    AssignmentBitwiseShiftLeft,
    AssignmentBitwiseShiftRight,

    // range
    Range,
    RangeInclusive,

    // binary logical
    Or,
    Xor,
    And,

    // comparison
    Equals,
    NotEquals,
    GreaterEquals,
    LesserEquals,
    Greater,
    Lesser,

    // binary bitwise
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    BitwiseShiftLeft,
    BitwiseShiftRight,

    // binary arithmetic
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,

    // type casting
    Casting,

    // unary logical
    Not,

    // unary bitwise
    BitwiseNot,

    // unary arithmetic
    Negation,

    // runtime access
    Index,
    Field,

    // call
    Call,

    // compile-time access
    Path,
}
