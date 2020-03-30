//!
//! The generator expression operand.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod constant;
pub mod list;
pub mod r#match;
pub mod memory;
pub mod structure;
pub mod tuple;
pub mod value;
pub mod variable;

use std::cell::RefCell;
use std::rc::Rc;

use crate::bytecode::Bytecode;
use crate::generator::expression::Expression;

use self::array::Expression as ArrayExpression;
use self::block::Expression as BlockExpression;
use self::conditional::Expression as ConditionalExpression;
use self::constant::Constant;
use self::list::Expression as ListExpression;
use self::memory::Memory;
use self::r#match::Expression as MatchExpression;
use self::structure::Expression as StructureExpression;
use self::tuple::Expression as TupleExpression;
use self::value::Value;
use self::variable::Variable;

#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Constant),
    Variable(Variable),
    Value(Value),
    Memory(Memory),
    Parenthesized(Box<Expression>),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    Structure(StructureExpression),
    List(ListExpression),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
}

impl Operand {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        match self {
            Self::Constant(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Variable(inner) => inner.write_all_to_bytecode_load(bytecode),
            Self::Value(_) => {}
            Self::Memory(inner) => inner.write_all_to_bytecode_load(bytecode),
            Self::Parenthesized(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Array(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Tuple(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Structure(inner) => inner.write_all_to_bytecode(bytecode),
            Self::List(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Block(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Conditional(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Match(inner) => inner.write_all_to_bytecode(bytecode),
        }
    }
}
