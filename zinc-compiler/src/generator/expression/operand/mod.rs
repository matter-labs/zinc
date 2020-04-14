//!
//! The generator expression operand.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod constant;
pub mod group;
pub mod list;
pub mod r#match;
pub mod place;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::generator::bytecode::Bytecode;

use self::array::Expression as ArrayExpression;
use self::block::Expression as BlockExpression;
use self::conditional::Expression as ConditionalExpression;
use self::constant::Constant;
use self::group::Expression as GroupExpression;
use self::list::Expression as ListExpression;
use self::place::Place;
use self::r#match::Expression as MatchExpression;

///
/// The expression operand which is translated to Zinc VM data.
///
#[derive(Debug, Clone)]
pub enum Operand {
    Constant(Constant),
    Place(Place),
    Array(ArrayExpression),
    Group(GroupExpression),
    List(ListExpression),
    Block(BlockExpression),
    Conditional(ConditionalExpression),
    Match(MatchExpression),
}

impl Operand {
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        match self {
            Self::Constant(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Place(inner) => {
                let location = inner.identifier.location;
                let is_place_indexed = !inner.elements.is_empty();
                let element_size = inner.element_size;
                let total_size = inner.total_size;

                let address = bytecode
                    .borrow()
                    .get_variable_address(inner.identifier.name.as_str())
                    .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                inner.write_all_to_bytecode(bytecode.clone());

                if is_place_indexed {
                    bytecode.borrow_mut().push_instruction(
                        Instruction::LoadSequenceByIndex(zinc_bytecode::LoadSequenceByIndex::new(
                            address,
                            total_size,
                            element_size,
                        )),
                        Some(location),
                    );
                } else {
                    bytecode.borrow_mut().push_instruction(
                        Instruction::LoadSequence(zinc_bytecode::LoadSequence::new(
                            address, total_size,
                        )),
                        Some(location),
                    );
                }
            }
            Self::Array(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Group(inner) => inner.write_all_to_bytecode(bytecode),
            Self::List(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Block(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Conditional(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Match(inner) => inner.write_all_to_bytecode(bytecode),
        }
    }
}
