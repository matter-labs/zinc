//!
//! The semantic analyzer scope type item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::element::r#type::Type as TypeElement;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::Scope;

#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be resolved during the second pass
    Unresolved {
        inner: TypeStatementVariant,
        scope: Rc<RefCell<Scope>>,
    },
    /// Resolved element ready to be used from anywhere
    Resolved {
        inner: TypeElement,
        intermediate: Option<GeneratorStatement>,
    },
}
