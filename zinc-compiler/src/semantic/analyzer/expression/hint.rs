//!
//! The expression translation hint.
//!

use crate::syntax::ExpressionOperator;

#[derive(Debug, Clone, Copy)]
pub enum Hint {
    // runtime
    PlaceExpression,
    ValueExpression,

    // compile time
    TypeExpression,
    PathExpression,
    CompoundTypeMember,
}

impl Hint {
    pub fn first(operator: ExpressionOperator) -> Self {
        match operator {
            ExpressionOperator::Assignment => Self::PlaceExpression,
            ExpressionOperator::AssignmentAddition => Self::PlaceExpression,
            ExpressionOperator::AssignmentSubtraction => Self::PlaceExpression,
            ExpressionOperator::AssignmentMultiplication => Self::PlaceExpression,
            ExpressionOperator::AssignmentDivision => Self::PlaceExpression,
            ExpressionOperator::AssignmentRemainder => Self::PlaceExpression,
            ExpressionOperator::AssignmentBitwiseOr => Self::PlaceExpression,
            ExpressionOperator::AssignmentBitwiseXor => Self::PlaceExpression,
            ExpressionOperator::AssignmentBitwiseAnd => Self::PlaceExpression,
            ExpressionOperator::AssignmentBitwiseShiftLeft => Self::PlaceExpression,
            ExpressionOperator::AssignmentBitwiseShiftRight => Self::PlaceExpression,

            ExpressionOperator::Range => Self::ValueExpression,
            ExpressionOperator::RangeInclusive => Self::ValueExpression,

            ExpressionOperator::Or => Self::ValueExpression,
            ExpressionOperator::Xor => Self::ValueExpression,
            ExpressionOperator::And => Self::ValueExpression,

            ExpressionOperator::Equals => Self::ValueExpression,
            ExpressionOperator::NotEquals => Self::ValueExpression,
            ExpressionOperator::GreaterEquals => Self::ValueExpression,
            ExpressionOperator::LesserEquals => Self::ValueExpression,
            ExpressionOperator::Greater => Self::ValueExpression,
            ExpressionOperator::Lesser => Self::ValueExpression,

            ExpressionOperator::BitwiseOr => Self::ValueExpression,
            ExpressionOperator::BitwiseXor => Self::ValueExpression,
            ExpressionOperator::BitwiseAnd => Self::ValueExpression,
            ExpressionOperator::BitwiseShiftLeft => Self::ValueExpression,
            ExpressionOperator::BitwiseShiftRight => Self::ValueExpression,

            ExpressionOperator::Addition => Self::ValueExpression,
            ExpressionOperator::Subtraction => Self::ValueExpression,
            ExpressionOperator::Multiplication => Self::ValueExpression,
            ExpressionOperator::Division => Self::ValueExpression,
            ExpressionOperator::Remainder => Self::ValueExpression,

            ExpressionOperator::Casting => Self::ValueExpression,

            ExpressionOperator::Not => Self::ValueExpression,

            ExpressionOperator::BitwiseNot => Self::ValueExpression,

            ExpressionOperator::Negation => Self::ValueExpression,

            ExpressionOperator::Index => Self::PlaceExpression,
            ExpressionOperator::Field => Self::PlaceExpression,

            ExpressionOperator::CallBuiltIn => Self::TypeExpression,
            ExpressionOperator::Call => Self::TypeExpression,

            ExpressionOperator::Path => Self::PathExpression,
        }
    }

    pub fn second(operator: ExpressionOperator) -> Self {
        match operator {
            ExpressionOperator::Assignment => Self::ValueExpression,
            ExpressionOperator::AssignmentAddition => Self::ValueExpression,
            ExpressionOperator::AssignmentSubtraction => Self::ValueExpression,
            ExpressionOperator::AssignmentMultiplication => Self::ValueExpression,
            ExpressionOperator::AssignmentDivision => Self::ValueExpression,
            ExpressionOperator::AssignmentRemainder => Self::ValueExpression,
            ExpressionOperator::AssignmentBitwiseOr => Self::ValueExpression,
            ExpressionOperator::AssignmentBitwiseXor => Self::ValueExpression,
            ExpressionOperator::AssignmentBitwiseAnd => Self::ValueExpression,
            ExpressionOperator::AssignmentBitwiseShiftLeft => Self::ValueExpression,
            ExpressionOperator::AssignmentBitwiseShiftRight => Self::ValueExpression,

            ExpressionOperator::Range => Self::ValueExpression,
            ExpressionOperator::RangeInclusive => Self::ValueExpression,

            ExpressionOperator::Or => Self::ValueExpression,
            ExpressionOperator::Xor => Self::ValueExpression,
            ExpressionOperator::And => Self::ValueExpression,

            ExpressionOperator::Equals => Self::ValueExpression,
            ExpressionOperator::NotEquals => Self::ValueExpression,
            ExpressionOperator::GreaterEquals => Self::ValueExpression,
            ExpressionOperator::LesserEquals => Self::ValueExpression,
            ExpressionOperator::Greater => Self::ValueExpression,
            ExpressionOperator::Lesser => Self::ValueExpression,

            ExpressionOperator::BitwiseOr => Self::ValueExpression,
            ExpressionOperator::BitwiseXor => Self::ValueExpression,
            ExpressionOperator::BitwiseAnd => Self::ValueExpression,
            ExpressionOperator::BitwiseShiftLeft => Self::ValueExpression,
            ExpressionOperator::BitwiseShiftRight => Self::ValueExpression,

            ExpressionOperator::Addition => Self::ValueExpression,
            ExpressionOperator::Subtraction => Self::ValueExpression,
            ExpressionOperator::Multiplication => Self::ValueExpression,
            ExpressionOperator::Division => Self::ValueExpression,
            ExpressionOperator::Remainder => Self::ValueExpression,

            ExpressionOperator::Casting => Self::TypeExpression,

            ExpressionOperator::Not => {
                panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS)
            }

            ExpressionOperator::BitwiseNot => {
                panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS)
            }

            ExpressionOperator::Negation => {
                panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS)
            }

            ExpressionOperator::Index => Self::ValueExpression,
            ExpressionOperator::Field => Self::CompoundTypeMember,

            ExpressionOperator::CallBuiltIn => {
                panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS)
            }
            ExpressionOperator::Call => Self::ValueExpression,

            ExpressionOperator::Path => Self::PathExpression,
        }
    }
}
