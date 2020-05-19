//!
//! The `enum` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a compile-time only enumeration type.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: EnumStatement) -> Result<Type, Error> {
        let r#type = Type::enumeration(
            statement.location,
            statement.identifier.name.clone(),
            statement.variants,
            Some(scope),
        )?;

        Ok(r#type)
    }
}
