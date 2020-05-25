//!
//! The expression operator.
//!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // assignment
    Assignment,
    AssignmentBitwiseOr,
    AssignmentBitwiseXor,
    AssignmentBitwiseAnd,
    AssignmentBitwiseShiftLeft,
    AssignmentBitwiseShiftRight,
    AssignmentAddition,
    AssignmentSubtraction,
    AssignmentMultiplication,
    AssignmentDivision,
    AssignmentRemainder,

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
    Dot,

    // call
    CallBuiltIn,
    Call,

    // compile-time access
    Path,

    // structure literal
    Structure,
}
