//!
//! The binding pattern builder.
//!

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
use crate::tree::pattern_binding::Pattern as BindingPattern;
use crate::tree::r#type::variant::Variant as TypeVariant;
use crate::tree::r#type::Type;

///
/// The binding pattern builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The binding pattern identifier.
    identifier: Option<Identifier>,
    /// The binding pattern type.
    r#type: Option<Type>,
    /// If the binding pattern is mutable.
    is_mutable: bool,
    /// If the binding pattern is a wildcard.
    is_wildcard: bool,
    /// If the binding pattern is a `self` alias.
    is_self_alias: bool,
    /// The location of the `self` alias.
    self_location: Option<Location>,
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
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_mutable(&mut self) {
        self.is_mutable = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_self_alias(&mut self) {
        self.is_self_alias = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_self_location(&mut self, value: Location) {
        self.self_location = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> BindingPattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let (variant, r#type) = if self.is_wildcard {
            let variant = BindingPatternVariant::new_wildcard();

            let r#type = self.r#type.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "type")
            });

            (variant, r#type)
        } else if self.is_self_alias {
            let self_location = self.self_location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "self location"
                )
            });

            let variant = BindingPatternVariant::new_self_alias(self_location, self.is_mutable);

            let r#type = Type::new(
                self_location,
                TypeVariant::alias(
                    ExpressionTree::new(
                        self_location,
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(self_location, Keyword::SelfUppercase.to_string()),
                        )),
                    ),
                    None,
                ),
            );

            (variant, r#type)
        } else if let Some(identifier) = self.identifier.take() {
            let variant = BindingPatternVariant::new_binding(identifier, self.is_mutable);

            let r#type = self.r#type.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "type")
            });

            (variant, r#type)
        } else {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "identifier | self | wildcard"
            );
        };

        BindingPattern::new(location, variant, r#type)
    }
}
