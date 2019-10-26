//!
//! The match expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::MatchExpression;
use crate::syntax::Pattern;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    match_expression: Option<Expression>,
    branches: Vec<(Pattern, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_match_expression(&mut self, value: Expression) {
        self.match_expression = Some(value);
    }

    pub fn push_branch_pattern(&mut self, value: Pattern) {
        self.branches.push((value, None));
    }

    pub fn set_branch_expression(&mut self, value: Expression) {
        self.branches
            .last_mut()
            .expect("Missing branch expression")
            .1 = Some(value);
    }

    pub fn finish(self) -> MatchExpression {
        MatchExpression::new(
            self.location.expect("Missing location"),
            self.match_expression.expect("Missing match expression"),
            self.branches
                .into_iter()
                .map(|(pattern, expression)| {
                    (pattern, expression.expect("Missing branch expression"))
                })
                .collect::<Vec<(Pattern, Expression)>>(),
        )
    }
}
