//!
//! The expression operator.
//!

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    // assignment
    Assignment,

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

    // type semantic.casting
    Casting,

    // unary logical
    Not,

    // unary arithmetic
    Negation,

    // access
    Indexing,
    Field,
    Call,
    InstructionCall,
    Path,
}
