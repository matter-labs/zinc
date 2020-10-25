//!
//! The `fn` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::attribute::Attribute;
use crate::tree::binding::Binding;
use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::identifier::Identifier;
use crate::tree::r#type::Type;
use crate::tree::statement::r#fn::Statement as FnStatement;

///
/// The `fn` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// If the function is public.
    is_public: bool,
    /// If the function is constant.
    is_constant: bool,
    /// The function identifier.
    identifier: Option<Identifier>,
    /// The function argument bindings.
    argument_bindings: Vec<Binding>,
    /// The optional function return type, which is `()` if not specified.
    return_type: Option<Type>,
    /// The function block.
    body: Option<BlockExpression>,
    /// The function outer attributes.
    attributes: Vec<Attribute>,
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
    pub fn set_public(&mut self) {
        self.is_public = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_constant(&mut self) {
        self.is_constant = true;
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
    pub fn set_argument_bindings(&mut self, value: Vec<Binding>) {
        self.argument_bindings = value;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_return_type(&mut self, value: Type) {
        self.return_type = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_body(&mut self, value: BlockExpression) {
        self.body = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_attributes(&mut self, value: Vec<Attribute>) {
        self.attributes = value;
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> FnStatement {
        FnStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.is_public,
            self.is_constant,
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.argument_bindings,
            self.return_type.take(),
            self.body.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "body")
            }),
            self.attributes,
        )
    }
}
