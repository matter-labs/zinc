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

    // unary arithmetic
    Negation,

    // access
    Index,
    Field,
    Path,

    // call
    Call,
}
