//!
//! The generator expression match operand builder.
//!

use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::operand::r#match::Expression as MatchExpression;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    scrutinee: Option<GeneratorExpression>,
    branches: Vec<(Constant, GeneratorExpression)>,
    else_branch: Option<GeneratorExpression>,
}

impl Builder {
    pub fn set_scrutinee(&mut self, value: GeneratorExpression) {
        self.scrutinee = Some(value);
    }

    pub fn push_branch(&mut self, pattern: Constant, expression: GeneratorExpression) {
        self.branches.push((pattern, expression));
    }

    pub fn set_else_branch(&mut self, value: GeneratorExpression) {
        self.else_branch = Some(value);
    }

    pub fn finish(mut self) -> MatchExpression {
        let scrutinee = self
            .scrutinee
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "scrutinee"));

        let else_branch = self
            .else_branch
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "else_branch"));

        MatchExpression::new(scrutinee, self.branches, else_branch)
    }
}
