//!
//! The expression evaluation rule.
//!

use zinc_syntax::ExpressionOperator;

///
/// The evaluation rule.
///
/// Used by some operators for turning path expressions to place or type ones,
/// or place ones to value ones.
///
#[derive(Debug, Clone, Copy)]
pub enum Rule {
    /// Describes a runtime memory location.
    Place,
    /// Describes a runtime value.
    Value,

    /// Describes a compile-time constant.
    Constant,
    /// Describes a compile-time type.
    Type,
    /// Describes a compile-time path.
    Path,
    /// Describes a compile-time field identifier or path element.
    Field,
}

impl Rule {
    ///
    /// Returns the first operand translation rule required by `operator`.
    ///
    /// The `rule` arguments contains an initial bias which helps to make the final decision.
    ///
    pub fn first(operator: ExpressionOperator, rule: Self) -> Self {
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

            ExpressionOperator::Range => rule.constant_or_value(),
            ExpressionOperator::RangeInclusive => rule.constant_or_value(),

            ExpressionOperator::Or => rule.constant_or_value(),
            ExpressionOperator::Xor => rule.constant_or_value(),
            ExpressionOperator::And => rule.constant_or_value(),

            ExpressionOperator::Equals => rule.constant_or_value(),
            ExpressionOperator::NotEquals => rule.constant_or_value(),
            ExpressionOperator::GreaterEquals => rule.constant_or_value(),
            ExpressionOperator::LesserEquals => rule.constant_or_value(),
            ExpressionOperator::Greater => rule.constant_or_value(),
            ExpressionOperator::Lesser => rule.constant_or_value(),

            ExpressionOperator::BitwiseOr => rule.constant_or_value(),
            ExpressionOperator::BitwiseXor => rule.constant_or_value(),
            ExpressionOperator::BitwiseAnd => rule.constant_or_value(),
            ExpressionOperator::BitwiseShiftLeft => rule.constant_or_value(),
            ExpressionOperator::BitwiseShiftRight => rule.constant_or_value(),

            ExpressionOperator::Addition => rule.constant_or_value(),
            ExpressionOperator::Subtraction => rule.constant_or_value(),
            ExpressionOperator::Multiplication => rule.constant_or_value(),
            ExpressionOperator::Division => rule.constant_or_value(),
            ExpressionOperator::Remainder => rule.constant_or_value(),

            ExpressionOperator::Casting => rule.constant_or_value(),

            ExpressionOperator::Not => rule.constant_or_value(),
            ExpressionOperator::BitwiseNot => rule.constant_or_value(),
            ExpressionOperator::Negation => rule.constant_or_value(),

            ExpressionOperator::Index => rule.constant_or_place(),
            ExpressionOperator::Dot => rule.constant_or_place(),

            ExpressionOperator::CallIntrinsic => Self::Type,
            ExpressionOperator::Call => Self::Type,

            ExpressionOperator::Path => Self::Path,

            ExpressionOperator::Structure => Self::Type,
        }
    }

    ///
    /// Returns the second operand translation rule required by `operator`.
    ///
    /// The `rule` arguments contains an initial bias which helps to make the final decision.
    ///
    pub fn second(operator: ExpressionOperator, rule: Self) -> Self {
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

            ExpressionOperator::Range => rule.constant_or_value(),
            ExpressionOperator::RangeInclusive => rule.constant_or_value(),

            ExpressionOperator::Or => rule.constant_or_value(),
            ExpressionOperator::Xor => rule.constant_or_value(),
            ExpressionOperator::And => rule.constant_or_value(),

            ExpressionOperator::Equals => rule.constant_or_value(),
            ExpressionOperator::NotEquals => rule.constant_or_value(),
            ExpressionOperator::GreaterEquals => rule.constant_or_value(),
            ExpressionOperator::LesserEquals => rule.constant_or_value(),
            ExpressionOperator::Greater => rule.constant_or_value(),
            ExpressionOperator::Lesser => rule.constant_or_value(),

            ExpressionOperator::BitwiseOr => rule.constant_or_value(),
            ExpressionOperator::BitwiseXor => rule.constant_or_value(),
            ExpressionOperator::BitwiseAnd => rule.constant_or_value(),
            ExpressionOperator::BitwiseShiftLeft => rule.constant_or_value(),
            ExpressionOperator::BitwiseShiftRight => rule.constant_or_value(),

            ExpressionOperator::Addition => rule.constant_or_value(),
            ExpressionOperator::Subtraction => rule.constant_or_value(),
            ExpressionOperator::Multiplication => rule.constant_or_value(),
            ExpressionOperator::Division => rule.constant_or_value(),
            ExpressionOperator::Remainder => rule.constant_or_value(),

            ExpressionOperator::Casting => Self::Type,

            ExpressionOperator::Not => panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS),
            ExpressionOperator::BitwiseNot => {
                panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
            }
            ExpressionOperator::Negation => {
                panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
            }

            ExpressionOperator::Index => rule.constant_or_value(),
            ExpressionOperator::Dot => Self::Field,

            ExpressionOperator::CallIntrinsic => {
                panic!(zinc_const::panic::VALIDATED_DURING_SYNTAX_ANALYSIS)
            }
            ExpressionOperator::Call => rule.constant_or_value(),

            ExpressionOperator::Path => Self::Path,

            ExpressionOperator::Structure => rule.constant_or_value(),
        }
    }

    ///
    /// If `self` is `Constant`, returns `Constant`, otherwise, returns `Value`.
    ///
    /// Is used to turn a value rule into a constant one in constant expression contexts.
    ///
    pub fn constant_or_value(self) -> Self {
        match self {
            Self::Constant => Self::Constant,
            _rule => Self::Value,
        }
    }

    ///
    /// If `self` is `Constant`, returns `Constant`, otherwise, returns `Place`.
    ///
    /// Is used to turn a place rule into a constant one in constant expression contexts.
    ///
    pub fn constant_or_place(self) -> Self {
        match self {
            Self::Constant => Self::Constant,
            _rule => Self::Place,
        }
    }
}
