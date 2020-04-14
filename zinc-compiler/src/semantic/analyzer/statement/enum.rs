//!
//! The `enum` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::r#type::Type;
use crate::semantic::element::r#type::INDEX as TYPE_INDEX;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#enum::Statement as EnumStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only enumeration declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: EnumStatement) -> Result<(), Error> {
        let unique_id = TYPE_INDEX.read().expect(crate::panic::MUTEX_SYNC).len();
        let r#type = Type::enumeration(
            statement.identifier.clone(),
            unique_id,
            statement.variants,
            Some(Rc::new(RefCell::new(Scope::new(Some(scope.clone()))))),
        )?;

        TYPE_INDEX
            .write()
            .expect(crate::panic::MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());

        Scope::declare_type(scope, statement.identifier, r#type)?;

        Ok(())
    }
}
