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

use self::contract::Statement as ContractStatement;
use self::r#fn::Statement as FnStatement;
use self::r#for::Statement as ForStatement;
use self::r#let::Statement as LetStatement;

///
/// Statements translated to the target Zinc VM bytecode.
///
#[derive(Debug, Clone)]
pub enum Statement {
    Fn(FnStatement),
    Let(LetStatement),
    Contract(ContractStatement),
    For(ForStatement),
    Expression(Expression),
}

impl Statement {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        match self {
            Self::Fn(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Let(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Contract(inner) => inner.write_all_to_bytecode(bytecode),
            Self::For(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Expression(inner) => inner.write_all_to_bytecode(bytecode),
        }
    }
}
