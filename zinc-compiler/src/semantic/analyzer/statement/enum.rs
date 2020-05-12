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
    /// Analyzes a compile-time only enumeration declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: EnumStatement) -> Result<Type, Error> {
        let r#type = Type::enumeration(
            statement.location,
            statement.identifier.name,
            statement.variants,
            Some(Scope::new_child(scope)),
        )?;

        Ok(r#type)
    }
}
