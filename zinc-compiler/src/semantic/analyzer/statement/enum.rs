//!
//! The `enum` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::EnumStatement;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The `enum` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a compile-time only enumeration type.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: EnumStatement) -> Result<Type, Error> {
        let r#type = Type::enumeration(
            statement.location,
            statement.identifier.name,
            statement.variants,
            vec![],
            scope,
        )?;

        Ok(r#type)
    }
}
