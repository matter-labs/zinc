//!
//! The intermediate representation for Zinc VM bytecode generating.
//!

pub mod expression;
pub mod module;
pub mod state;
pub mod statement;
pub mod r#type;

use std::cell::RefCell;
use std::rc::Rc;

use self::state::State;

///
/// Implemented by items which are translated into the Zinc VM bytecode.
///
pub trait IBytecodeWritable {
    ///
    /// Writes the item to the bytecode generator state object.
    ///
    fn write_all(self, bytecode: Rc<RefCell<State>>);
}
