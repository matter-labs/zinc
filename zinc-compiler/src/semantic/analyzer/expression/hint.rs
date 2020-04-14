//!
//! The expression evaluation hint.
//!

use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;

///
/// The evaluation hint.
///
/// Used by some operators for turning path expressions to place or type ones,
/// or place ones to value ones.
///
#[derive(Debug, Clone, Copy)]
pub enum Hint {
    /// describes a runtime memory location
    Place,
    /// describes a runtime or compile-time value (will be separated later)
    Value,
    /// describes a compile-time type
    Type,
    /// describes a compile-time path
    Path,
    /// describes a compile-time field identifier or path element
    Field,
}

impl Hint {
    ///
    /// Returns the first operand expression type required by `operator`.
    ///
    pub fn first(operator: ExpressionOperator) -> Self {
        match operator {
            ExpressionOperator::Assignment => Self::Place,
            ExpressionOperator::AssignmentAddition => Self::Place,
            ExpressionOperator::AssignmentSubtraction => Self::Place,
            ExpressionOperator::AssignmentMultiplication => Self::Place,
            ExpressionOperator::AssignmentDivision => Self::Place,
            ExpressionOperator::AssignmentRemainder => Self::Place,
            ExpressionOperator::AssignmentBitwiseOr => Self::Place,
            ExpressionOperator::AssignmentBitwiseXor => Self::Place,
            ExpressionOperator::AssignmentBitwiseAnd => Self::Place,
            ExpressionOperator::AssignmentBitwiseShiftLeft => Self::Place,
            ExpressionOperator::AssignmentBitwiseShiftRight => Self::Place,

            ExpressionOperator::Range => Self::Value,
            ExpressionOperator::RangeInclusive => Self::Value,

            ExpressionOperator::Or => Self::Value,
            ExpressionOperator::Xor => Self::Value,
            ExpressionOperator::And => Self::Value,

            ExpressionOperator::Equals => Self::Value,
            ExpressionOperator::NotEquals => Self::Value,
            ExpressionOperator::GreaterEquals => Self::Value,
            ExpressionOperator::LesserEquals => Self::Value,
            ExpressionOperator::Greater => Self::Value,
            ExpressionOperator::Lesser => Self::Value,

            ExpressionOperator::BitwiseOr => Self::Value,
            ExpressionOperator::BitwiseXor => Self::Value,
            ExpressionOperator::BitwiseAnd => Self::Value,
            ExpressionOperator::BitwiseShiftLeft => Self::Value,
            ExpressionOperator::BitwiseShiftRight => Self::Value,

            ExpressionOperator::Addition => Self::Value,
            ExpressionOperator::Subtraction => Self::Value,
            ExpressionOperator::Multiplication => Self::Value,
            ExpressionOperator::Division => Self::Value,
            ExpressionOperator::Remainder => Self::Value,

            ExpressionOperator::Casting => Self::Value,

            ExpressionOperator::Not => Self::Value,
            ExpressionOperator::BitwiseNot => Self::Value,
            ExpressionOperator::Negation => Self::Value,

            ExpressionOperator::Index => Self::Place,
            ExpressionOperator::Field => Self::Place,

            ExpressionOperator::CallBuiltIn => Self::Type,
            ExpressionOperator::Call => Self::Type,

            ExpressionOperator::Path => Self::Path,
        }
    }

    ///
    /// Returns the second operand expression type required by `operator`.
    ///
    pub fn second(operator: ExpressionOperator) -> Self {
        match operator {
            ExpressionOperator::Assignment => Self::Value,
            ExpressionOperator::AssignmentAddition => Self::Value,
            ExpressionOperator::AssignmentSubtraction => Self::Value,
            ExpressionOperator::AssignmentMultiplication => Self::Value,
            ExpressionOperator::AssignmentDivision => Self::Value,
            ExpressionOperator::AssignmentRemainder => Self::Value,
            ExpressionOperator::AssignmentBitwiseOr => Self::Value,
            ExpressionOperator::AssignmentBitwiseXor => Self::Value,
            ExpressionOperator::AssignmentBitwiseAnd => Self::Value,
            ExpressionOperator::AssignmentBitwiseShiftLeft => Self::Value,
            ExpressionOperator::AssignmentBitwiseShiftRight => Self::Value,

            ExpressionOperator::Range => Self::Value,
            ExpressionOperator::RangeInclusive => Self::Value,

            ExpressionOperator::Or => Self::Value,
            ExpressionOperator::Xor => Self::Value,
            ExpressionOperator::And => Self::Value,

            ExpressionOperator::Equals => Self::Value,
            ExpressionOperator::NotEquals => Self::Value,
            ExpressionOperator::GreaterEquals => Self::Value,
            ExpressionOperator::LesserEquals => Self::Value,
            ExpressionOperator::Greater => Self::Value,
            ExpressionOperator::Lesser => Self::Value,

            ExpressionOperator::BitwiseOr => Self::Value,
            ExpressionOperator::BitwiseXor => Self::Value,
            ExpressionOperator::BitwiseAnd => Self::Value,
            ExpressionOperator::BitwiseShiftLeft => Self::Value,
            ExpressionOperator::BitwiseShiftRight => Self::Value,

            ExpressionOperator::Addition => Self::Value,
            ExpressionOperator::Subtraction => Self::Value,
            ExpressionOperator::Multiplication => Self::Value,
            ExpressionOperator::Division => Self::Value,
            ExpressionOperator::Remainder => Self::Value,

            ExpressionOperator::Casting => Self::Type,

            ExpressionOperator::Not => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
            ExpressionOperator::BitwiseNot => {
                panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
            }
            ExpressionOperator::Negation => panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),

            ExpressionOperator::Index => Self::Value,
            ExpressionOperator::Field => Self::Field,

            ExpressionOperator::CallBuiltIn => {
                panic!(crate::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
            }
            ExpressionOperator::Call => Self::Value,

            ExpressionOperator::Path => Self::Path,
        }
    }
}
