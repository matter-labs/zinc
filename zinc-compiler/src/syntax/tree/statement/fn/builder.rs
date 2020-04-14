//!
//! The fn statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::Pattern as BindingPattern;
use crate::syntax::tree::r#type::Type;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    argument_bindings: Vec<BindingPattern>,
    return_type: Option<Type>,
    body: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_argument_bindings(&mut self, value: Vec<BindingPattern>) {
        self.argument_bindings = value;
    }

    pub fn set_return_type(&mut self, value: Type) {
        self.return_type = Some(value);
    }

    pub fn set_body(&mut self, value: BlockExpression) {
        self.body = Some(value);
    }

    pub fn finish(mut self) -> FnStatement {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location"));
        FnStatement::new(
            location,
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.argument_bindings,
            self.return_type.take(),
            self.body
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "body")),
        )
    }
}
