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
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;

use self::contract::Statement as ContractStatement;
use self::r#fn::Statement as FnStatement;
use self::r#for::Statement as ForStatement;
use self::r#let::Statement as LetStatement;

///
/// Statements translated to the target Zinc VM bytecode.
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
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        match self {
            Self::Fn(inner) => inner.write_all(bytecode),
            Self::Let(inner) => inner.write_all(bytecode),
            Self::Contract(inner) => inner.write_all(bytecode),
            Self::For(inner) => inner.write_all(bytecode),
            Self::Expression(inner) => inner.write_all(bytecode),
        }
    }
}
