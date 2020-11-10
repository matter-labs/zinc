//!
//! The `type` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::TypeStatement;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

///
/// The `type` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a compile-time only type alias.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: TypeStatement) -> Result<Type, Error> {
        let r#type = Type::try_from_syntax(statement.r#type, scope)?;

        if !r#type.is_instantiatable(false) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement.location,
                found: r#type.to_string(),
            });
        }

        Ok(r#type)
    }
}
