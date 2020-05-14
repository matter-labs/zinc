//!
//! The generator expression place operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::Instruction;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::operand::constant::Constant;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::constant::Constant as SemanticConstant;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;
use crate::semantic::element::place::Place as SemanticPlace;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct Place {
    pub identifier: Identifier,
    pub element_size: usize,
    pub total_size: usize,
    pub elements: Vec<SemanticPlaceElement>,
    pub memory_type: MemoryType,
}

impl Place {
    pub fn load_storage(&mut self, bytecode: Rc<RefCell<Bytecode>>) -> usize {
        if let Some(SemanticPlaceElement::ContractField {
            access:
                ContractFieldAccess {
                    position,
                    element_size,
                },
        }) = self.elements.first()
        {
            let position = *position;

            IntegerConstant::new(BigInt::from(position), false, crate::BITLENGTH_FIELD)
                .write_all_to_bytecode(bytecode.clone());
            bytecode.borrow_mut().push_instruction(
                Instruction::StorageLoad(zinc_bytecode::StorageLoad::new(*element_size)),
                Some(self.identifier.location),
            );

            self.elements.remove(0);

            position
        } else {
            panic!(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        if !self.elements.is_empty() {
            IntegerConstant::new(BigInt::zero(), false, crate::BITLENGTH_FIELD)
                .write_all_to_bytecode(bytecode.clone());
        }

        for element in self.elements.into_iter() {
            match element {
                SemanticPlaceElement::IndexConstant { constant, access } => {
                    Constant::try_from_semantic(&SemanticConstant::Integer(constant))
                        .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                        .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.identifier.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexExpression { expression, access } => {
                    expression.write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_bytecode::Mul),
                        Some(self.identifier.location),
                    );
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRange { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRangeInclusive { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::StackField { access } => {
                    IntegerConstant::new(
                        BigInt::from(access.offset),
                        false,
                        crate::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::Add(zinc_bytecode::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::ContractField { .. } => {}
            }
        }
    }
}

impl From<SemanticPlace> for Place {
    fn from(place: SemanticPlace) -> Self {
        Self {
            identifier: place.identifier,
            element_size: place.r#type.size(),
            total_size: place.total_size,
            elements: place.elements,
            memory_type: place.memory_type,
        }
    }
}
