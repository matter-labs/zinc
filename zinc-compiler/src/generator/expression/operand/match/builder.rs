//!
//! The generator expression match operand builder.
//!

use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::operand::r#match::Expression as MatchExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use zinc_lexical::Location;

///
/// The generator expression match operand builder.
///
#[derive(Debug, Default, Clone)]
pub struct Builder {
    /// The `match` expression location.
    location: Option<Location>,
    /// The scrutinee (matched) expression.
    scrutinee: Option<GeneratorExpression>,
    /// The scrutinee (matched) expression type.
    scrutinee_type: Option<Type>,
    /// The branches ordered array, where each branch consists of a pattern and result expression.
    branches: Vec<(Constant, GeneratorExpression)>,
    /// The binding branch, which is the last fallback branch.
    binding_branch: Option<(GeneratorExpression, String)>,
    /// The wildcard `_` branch, which is the last fallback branch. Ignored if `binding_branch` is set.
    wildcard_branch: Option<GeneratorExpression>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_scrutinee(&mut self, value: GeneratorExpression, r#type: Type) {
        self.scrutinee = Some(value);
        self.scrutinee_type = Some(r#type);
    }

    ///
    /// Pushes a branch, which consists of a `pattern` and `expression`.
    ///
    pub fn push_branch(&mut self, pattern: Constant, expression: GeneratorExpression) {
        self.branches.push((pattern, expression));
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_binding_branch(&mut self, expression: GeneratorExpression, name: String) {
        self.binding_branch = Some((expression, name));
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_wildcard_branch(&mut self, value: GeneratorExpression) {
        self.wildcard_branch = Some(value);
    }

    ///
    /// Finilizes the builder and returns the built item.
    ///
    pub fn finish(mut self) -> MatchExpression {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let scrutinee = self.scrutinee.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "scrutinee"
            )
        });
        let scrutinee_type = self.scrutinee_type.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "scrutinee type"
            )
        });

        match self.binding_branch.take() {
            Some(binding_branch) => MatchExpression::new(
                location,
                scrutinee,
                scrutinee_type,
                self.branches,
                Some(binding_branch),
                None,
            ),
            None => {
                let wildcard_branch = self.wildcard_branch.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        zinc_const::panic::BUILDER_REQUIRES_VALUE,
                        "wildcard branch"
                    )
                });
                MatchExpression::new(
                    location,
                    scrutinee,
                    scrutinee_type,
                    self.branches,
                    None,
                    Some(wildcard_branch),
                )
            }
        }
    }
}
