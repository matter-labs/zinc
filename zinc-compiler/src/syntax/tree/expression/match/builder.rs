//!
//! The match expression builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::r#match::Expression as MatchExpression;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    scrutinee: Option<Expression>,
    branches: Vec<(MatchPattern, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_scrutinee_expression(&mut self, value: Expression) {
        self.scrutinee = Some(value);
    }

    pub fn push_branch_pattern(&mut self, value: MatchPattern) {
        self.branches.push((value, None));
    }

    pub fn set_branch_expression(&mut self, value: Expression) {
        self.branches
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "branch expression"
                )
            })
            .1 = Some(value);
    }

    pub fn finish(self) -> MatchExpression {
        MatchExpression::new(
            self.location.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.scrutinee.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "scrutinee"
                )
            }),
            self.branches
                .into_iter()
                .map(|(pattern, expression)| {
                    (
                        pattern,
                        expression.unwrap_or_else(|| {
                            panic!(
                                "{}{}",
                                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                                "branch expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(MatchPattern, Expression)>>(),
        )
    }
}
