//!
//! The generator statement.
//!

pub mod declaration;
pub mod function;
pub mod loop_for;

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::Expression;
use crate::generator::statement::declaration::Statement as DeclarationStatement;
use crate::generator::statement::function::Statement as FunctionStatement;
use crate::generator::statement::loop_for::Statement as ForLoopStatement;

///
/// Statements translated to the target Zinc VM bytecode.
///
#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Declaration(DeclarationStatement),
    Loop(ForLoopStatement),
    Function(FunctionStatement),
    Implementation(Vec<Self>),
    Contract(Vec<Self>),
}

impl Statement {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        match self {
            Self::Expression(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Declaration(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Loop(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Function(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Implementation(inner) => {
                for element in inner.into_iter() {
                    element.write_all_to_bytecode(bytecode.clone());
                }
            }
            Self::Contract(inner) => {
                for element in inner.into_iter() {
                    element.write_all_to_bytecode(bytecode.clone());
                }
            }
        }
    }
}
