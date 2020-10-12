//!
//! The semantic analyzer scope constant item state.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ConstStatement;

use crate::semantic::element::constant::Constant as ConstantElement;
use crate::semantic::scope::Scope;

///
/// The definition state, which is either `declared` or `defined`.
///
#[derive(Debug, Clone)]
pub enum State {
    /// Waiting to be defined during the second pass.
    Declared {
        /// The constant syntax representation.
        inner: ConstStatement,
        /// The scope, where the constant is declared and must be defined later.
        scope: Rc<RefCell<Scope>>,
    },
    /// Defined element ready to be used from anywhere.
    Defined {
        /// The semantic constant element.
        inner: ConstantElement,
    },
}
