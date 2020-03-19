//!
//! The generator expression operator.
//!

#[derive(Debug, Clone)]
pub enum Operator {
    // assignment
    Assignment,
    AssignmentAddition,
    AssignmentSubtraction,
    AssignmentMultiplication,
    AssignmentDivision,
    AssignmentRemainder,

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
    ArrayIndex,
    Slice,

    // call
    Call,
}
