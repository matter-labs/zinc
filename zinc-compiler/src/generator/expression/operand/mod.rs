//!
//! The generator expression operand.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod constant;
pub mod list;
pub mod r#match;
pub mod structure;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::expression::Expression;
use crate::semantic::element::place::Place;

use self::array::Expression as ArrayExpression;
use self::block::Expression as BlockExpression;
use self::conditional::Expression as ConditionalExpression;
use self::constant::Constant;
use self::list::Expression as ListExpression;
use self::r#match::Expression as MatchExpression;
use self::structure::Expression as StructureExpression;
use self::tuple::Expression as TupleExpression;

#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Constant),
    Place(Place),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    Structure(StructureExpression),
    List(ListExpression),
    Parenthesized(Box<Expression>),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
}

impl Operand {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        match self {
            Self::Constant(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Place(inner) => {
                let address = bytecode
                    .borrow()
                    .get_variable_address(inner.identifier.as_str())
                    .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS);
                bytecode.borrow_mut().push_instruction(
                    Instruction::Load(zinc_bytecode::Load::new(address)),
                    crate::lexical::Location::default(),
                );
            }
            Self::Array(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Tuple(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Structure(inner) => inner.write_all_to_bytecode(bytecode),
            Self::List(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Parenthesized(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Block(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Conditional(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Match(inner) => inner.write_all_to_bytecode(bytecode),
        }
    }
}
