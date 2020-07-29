//!
//! The `type` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

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

        Ok(r#type)
    }
}
