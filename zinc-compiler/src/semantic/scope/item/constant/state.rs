//!
//! The semantic analyzer scope constant item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::element::constant::Constant as ConstantElement;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be resolved during the second pass
    Unresolved {
        inner: ConstStatement,
        scope: Rc<RefCell<Scope>>,
    },
    /// Resolved element ready to be used from anywhere
    Resolved { inner: ConstantElement },
}
