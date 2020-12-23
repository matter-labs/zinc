//!
//! The generator statement.
//!

pub mod contract;
pub mod r#fn;
pub mod r#for;
pub mod r#let;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::expression::Expression;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;

use self::contract::Statement as ContractStatement;
use self::r#fn::Statement as FnStatement;
use self::r#for::Statement as ForStatement;
use self::r#let::Statement as LetStatement;

///
/// The generator statement.
///
#[derive(Debug, Clone)]
pub enum Statement {
    /// The `fn` statement.
    Fn(FnStatement),
    /// The `let` statement.
    Let(LetStatement),
    /// The `contract` statement.
    Contract(ContractStatement),
    /// The `for` statement.
    For(ForStatement),
    /// The expression statement, which is actually a large class of expression-like statements.
    Expression(Expression),
}

impl IBytecodeWritable for Statement {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        match self {
            Self::Fn(inner) => inner.write_to_zinc_vm(state),
            Self::Let(inner) => inner.write_to_zinc_vm(state),
            Self::Contract(inner) => inner.write_to_zinc_vm(state),
            Self::For(inner) => inner.write_to_zinc_vm(state),
            Self::Expression(inner) => inner.write_to_zinc_vm(state),
        }
    }
}
