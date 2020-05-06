//!
//! The `enum` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only enumeration declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: EnumStatement) -> Result<(), Error> {
        let location = statement.location;

        let r#type = Type::enumeration(
            statement.location,
            statement.identifier.name.clone(),
            statement.variants,
            Some(Rc::new(RefCell::new(Scope::new(Some(scope.clone()))))),
        )?;

        Scope::declare_type(
            scope,
            statement.identifier,
            ScopeTypeItem::new(Some(location), r#type),
        )?;

        Ok(())
    }
}
