//!
//! The match expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::r#match::Expression as MatchExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::pattern_match::Pattern as MatchPattern;

///
/// The match expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The match scrutinee expression, which is the matched expression.
    scrutinee: Option<ExpressionTree>,
    /// The match pattern-expression pairs.
    branches: Vec<(MatchPattern, Option<ExpressionTree>)>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_scrutinee_expression(&mut self, value: ExpressionTree) {
        self.scrutinee = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_branch_pattern(&mut self, value: MatchPattern) {
        self.branches.push((value, None));
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_branch_expression(&mut self, value: ExpressionTree) {
        self.branches
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "branch pattern"
                )
            })
            .1 = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(self) -> MatchExpression {
        MatchExpression::new(
            self.location.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.scrutinee.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
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
                                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                                "branch expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(MatchPattern, ExpressionTree)>>(),
        )
    }
}
