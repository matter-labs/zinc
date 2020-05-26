//!
//! The generator expression match operand builder.
//!

use crate::generator::expression::operand::constant::Constant;
use crate::generator::expression::operand::r#match::Expression as MatchExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    location: Option<Location>,
    scrutinee: Option<GeneratorExpression>,
    scrutinee_type: Option<Type>,
    branches: Vec<(Constant, GeneratorExpression)>,
    binding_branch: Option<(GeneratorExpression, String)>,
    wildcard_branch: Option<GeneratorExpression>,
}

impl Builder {
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    pub fn set_scrutinee(&mut self, value: GeneratorExpression, r#type: Type) {
        self.scrutinee = Some(value);
        self.scrutinee_type = Some(r#type);
    }

    pub fn push_branch(&mut self, pattern: Constant, expression: GeneratorExpression) {
        self.branches.push((pattern, expression));
    }

    pub fn set_binding_branch(&mut self, expression: GeneratorExpression, name: String) {
        self.binding_branch = Some((expression, name));
    }

    pub fn set_wildcard_branch(&mut self, value: GeneratorExpression) {
        self.wildcard_branch = Some(value);
    }

    pub fn finish(mut self) -> MatchExpression {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        let scrutinee = self
            .scrutinee
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "scrutinee"));
        let scrutinee_type = self.scrutinee_type.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::panic::BUILDER_REQUIRES_VALUE,
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
                        crate::panic::BUILDER_REQUIRES_VALUE,
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
