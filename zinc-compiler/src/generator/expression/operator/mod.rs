//!
//! The generator expression operator.
//!

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::generator::r#type::Type;
use crate::semantic::Type as SemanticType;

#[derive(Debug, Clone)]
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
