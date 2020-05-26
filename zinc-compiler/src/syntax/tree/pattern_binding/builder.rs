//!
//! The binding pattern builder.
//!

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
use crate::syntax::tree::r#type::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    r#type: Option<Type>,
    is_mutable: bool,
    is_wildcard: bool,
    is_self_alias: bool,
    self_location: Option<Location>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn set_is_mutable(&mut self) {
        self.is_mutable = true;
    }

    pub fn set_is_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    pub fn set_is_self_alias(&mut self) {
        self.is_self_alias = true;
    }

    pub fn set_self_location(&mut self, value: Location) {
        self.self_location = Some(value);
    }

    pub fn finish(mut self) -> BindingPattern {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));

        let (variant, r#type) =
            if self.is_wildcard {
                let variant = BindingPatternVariant::new_wildcard();

                let r#type = self.r#type.take().unwrap_or_else(|| {
                    panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "type")
                });

                (variant, r#type)
            } else if self.is_self_alias {
                let self_location = self.self_location.take().unwrap_or_else(|| {
                    panic!(
                        "{}{}",
                        crate::panic::BUILDER_REQUIRES_VALUE,
                        "self location"
                    )
                });

                let variant = BindingPatternVariant::new_self_alias(self_location, self.is_mutable);

                let r#type = Type::new(
                    self_location,
                    TypeVariant::alias(ExpressionTree::new(
                        self_location,
                        ExpressionTreeNode::operand(ExpressionOperand::Identifier(
                            Identifier::new(self_location, Keyword::SelfUppercase.to_string()),
                        )),
                    )),
                );

                (variant, r#type)
            } else if let Some(identifier) = self.identifier.take() {
                let variant = BindingPatternVariant::new_binding(identifier, self.is_mutable);

                let r#type = self.r#type.take().unwrap_or_else(|| {
                    panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "type")
                });

                (variant, r#type)
            } else {
                panic!(
                    "{}{}",
                    crate::panic::BUILDER_REQUIRES_VALUE,
                    "identifier | self | wildcard"
                );
            };

        BindingPattern::new(location, variant, r#type)
    }
}
