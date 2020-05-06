//!
//! The `type` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#type::Statement as TypeStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only type alias declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: TypeStatement) -> Result<(), Error> {
        let location = statement.location;

        let r#type = Type::from_syntax_type(statement.r#type, scope.clone())?;

        Scope::declare_type(
            scope,
            statement.identifier,
            ScopeTypeItem::new(Some(location), r#type),
        )?;

        Ok(())
    }
}
