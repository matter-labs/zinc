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

use num_bigint::BigInt;

use zinc_bytecode::Instruction;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::state::State;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;

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
    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        match self {
            Self::Constant(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Place(mut inner) => match inner.memory_type {
                MemoryType::Stack => {
                    let location = inner.identifier.location;
                    let element_size = inner.element_size;
                    let total_size = inner.total_size;
                    let address = bytecode
                        .borrow()
                        .get_variable_address(inner.identifier.name.as_str())
                        .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                    let is_indexed = !inner.elements.is_empty();

                    if is_indexed {
                        inner.write_all_to_bytecode(bytecode.clone());
                        bytecode.borrow_mut().push_instruction(
                            Instruction::LoadByIndex(zinc_bytecode::LoadByIndex::new(
                                address,
                                element_size,
                                total_size,
                            )),
                            Some(location),
                        );
                    } else {
                        bytecode.borrow_mut().push_instruction(
                            Instruction::Load(zinc_bytecode::Load::new(address, total_size)),
                            Some(location),
                        );
                    }
                }
                MemoryType::ContractStorage => {
                    let location = inner.identifier.location;
                    let element_size = inner.element_size;
                    let total_size = inner.total_size;

                    if let Some(SemanticPlaceElement::ContractField {
                        access:
                            ContractFieldAccess {
                                position,
                                element_size,
                                ..
                            },
                    }) = inner.elements.first()
                    {
                        IntegerConstant::new(
                            BigInt::from(*position),
                            false,
                            zinc_const::bitlength::FIELD,
                        )
                        .write_all_to_bytecode(bytecode.clone());
                        bytecode.borrow_mut().push_instruction(
                            Instruction::StorageLoad(zinc_bytecode::StorageLoad::new(
                                *element_size,
                            )),
                            Some(inner.identifier.location),
                        );

                        inner.elements.remove(0);
                    }

                    let is_indexed = !inner.elements.is_empty();

                    if is_indexed {
                        inner.write_all_to_bytecode(bytecode.clone());
                        bytecode.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_bytecode::Slice::new(element_size, total_size)),
                            Some(location),
                        );
                    }
                }
            },
            Self::Array(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Group(inner) => inner.write_all_to_bytecode(bytecode),
            Self::List(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Block(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Conditional(inner) => inner.write_all_to_bytecode(bytecode),
            Self::Match(inner) => inner.write_all_to_bytecode(bytecode),
        }
    }
}
