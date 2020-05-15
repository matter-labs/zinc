//!
//! The `type` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only type alias declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: TypeStatement) -> Result<Type, Error> {
        let r#type = Type::from_syntax_type(statement.r#type, scope)?;

        Ok(r#type)
    }
}
