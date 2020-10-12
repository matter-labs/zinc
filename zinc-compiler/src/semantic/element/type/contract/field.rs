//!
//! The semantic analyzer type contract type storage field.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::FieldStatement;
use zinc_syntax::Identifier;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The semantic analyzer contract storage field representation.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// The field identifier with location.
    pub identifier: Identifier,
    /// The field type.
    pub r#type: Type,
    /// Whether the field is public.
    pub is_public: bool,
    /// Whether the field is implicit.
    pub is_implicit: bool,
    /// Whether the field is immutable.
    pub is_immutable: bool,
}

impl Field {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: Identifier,
        r#type: Type,
        is_public: bool,
        is_implicit: bool,
        is_immutable: bool,
    ) -> Self {
        Self {
            identifier,
            r#type,
            is_public,
            is_implicit,
            is_immutable,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn try_from_syntax(
        statement: FieldStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        let r#type = Type::try_from_syntax(statement.r#type, scope)?;

        Ok(Self {
            identifier: statement.identifier,
            r#type,
            is_public: statement.is_public,
            is_implicit: false,
            is_immutable: false,
        })
    }
}
