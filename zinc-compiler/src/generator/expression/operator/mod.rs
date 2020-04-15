//!
//! The generator expression operator.
//!

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::generator::expression::operand::place::Place;
use crate::generator::expression::Expression;
use crate::generator::r#type::Type;
use crate::semantic::element::access::Field as FieldAccess;
use crate::semantic::element::access::Index as IndexAccess;
use crate::semantic::element::r#type::Type as SemanticType;

///
/// The expression operator which is translated to some specific Zinc VM instructions.
///
#[derive(Debug, Clone)]
pub enum Operator {
    None,

    // assignment
    Assignment {
        place: Place,
        expression: Expression,
    },
    AssignmentBitwiseOr {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentBitwiseXor {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentBitwiseAnd {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentBitwiseShiftLeft {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentBitwiseShiftRight {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentAddition {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentSubtraction {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentMultiplication {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentDivision {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },
    AssignmentRemainder {
        place: Place,
        expression: Expression,
        operator: Box<Self>,
    },

    // binary logical
    Or,
    Xor,
    And,

    // binary comparison
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
    Casting {
        r#type: Type,
    },

    // unary logical
    Not,

    // unary bitwise
    BitwiseNot,

    // unary arithmetic
    Negation,

    // array index
    Index {
        expression: Expression,
        access: IndexAccess,
    },

    // tuple or structure slice
    Slice {
        access: FieldAccess,
    },

    // call
    Call {
        unique_id: usize,
        input_size: usize,
    },
    CallDebug {
        format: String,
        argument_types: Vec<Type>,
    },
    CallAssert {
        message: Option<String>,
    },
    CallStandardLibrary {
        identifier: BuiltinIdentifier,
        input_size: usize,
        output_size: usize,
    },
}

impl Operator {
    pub fn casting(r#type: &SemanticType) -> Option<Self> {
        Type::try_from_semantic(r#type).map(|r#type| Self::Casting { r#type })
    }

    pub fn index(expression: Expression, access: IndexAccess) -> Self {
        Self::Index { expression, access }
    }

    pub fn slice(access: FieldAccess) -> Self {
        Self::Slice { access }
    }

    pub fn call(unique_id: usize, input_size: usize) -> Self {
        Self::Call {
            unique_id,
            input_size,
        }
    }

    pub fn call_debug(format: String, argument_types: Vec<SemanticType>) -> Self {
        Self::CallDebug {
            format,
            argument_types: argument_types
                .as_slice()
                .iter()
                .filter_map(Type::try_from_semantic)
                .collect(),
        }
    }

    pub fn call_assert(message: Option<String>) -> Self {
        Self::CallAssert { message }
    }

    pub fn call_std(identifier: BuiltinIdentifier, input_size: usize, output_size: usize) -> Self {
        Self::CallStandardLibrary {
            identifier,
            input_size,
            output_size,
        }
    }
}
