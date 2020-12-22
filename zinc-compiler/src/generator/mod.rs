//!
//! The intermediate representation for bytecode generating.
//!

pub mod expression;
pub mod module;
pub mod statement;
pub mod r#type;
pub mod zinc_vm;

use std::cell::RefCell;
use std::rc::Rc;

use self::zinc_vm::State as ZincVMState;

///
/// Implemented by items which are translated into some target bytecode.
///
pub trait IBytecodeWritable {
    ///
    /// Writes the item to the Zinc VM bytecode generator state object.
    ///
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>);
}
