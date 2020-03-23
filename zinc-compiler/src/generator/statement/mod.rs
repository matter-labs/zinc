//!
//! The generator statement.
//!

pub mod declaration;
pub mod function;
pub mod loop_for;

use crate::generator::expression::Expression;
use crate::generator::statement::declaration::Statement as DeclarationStatement;
use crate::generator::statement::function::Statement as FunctionStatement;
use crate::generator::statement::loop_for::Statement as ForLoopStatement;

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    Declaration(DeclarationStatement),
    Loop(ForLoopStatement),
    Function(FunctionStatement),
    Implementation(Vec<Self>),
}
