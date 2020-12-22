//!
//! The generator expression place operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;
use num::Zero;

use zinc_syntax::Identifier;
use zinc_types::Instruction;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;
use crate::semantic::element::place::Place as SemanticPlace;

///
/// The bytecode generator memory place representation.
///
#[derive(Debug, Clone)]
pub struct Place {
    /// The memory place identifier, which is usually a variable name.
    pub identifier: Identifier,
    /// The inner element size, which is stored here when we get after going deeper into complex types,
    /// like arrays, tuples, or structures.
    pub element_size: usize,
    /// The variable total size, which is not changed during indexing.
    pub total_size: usize,
    /// The memory place path, which consists of array indexes and fields accesses.
    pub elements: Vec<SemanticPlaceElement>,
    /// The memory type, which the memory place is part of.
    pub memory_type: MemoryType,
}

impl IBytecodeWritable for Place {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        if !self.elements.is_empty() {
            IntegerConstant::new(BigInt::zero(), false, zinc_const::bitlength::FIELD)
                .write_to_zinc_vm(state.clone());
        }

        for element in self.elements.into_iter() {
            match element {
                SemanticPlaceElement::IndexConstant { constant, access } => {
                    IntegerConstant::from_semantic(&constant).write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_types::Cast::new(zinc_types::ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_types::Mul),
                        Some(self.identifier.location),
                    );
                    state.borrow_mut().push_instruction(
                        Instruction::Add(zinc_types::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexExpression { expression, access } => {
                    expression.write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Cast(zinc_types::Cast::new(zinc_types::ScalarType::Field)),
                        Some(self.identifier.location),
                    );
                    IntegerConstant::new(
                        BigInt::from(access.element_size),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Mul(zinc_types::Mul),
                        Some(self.identifier.location),
                    );
                    state.borrow_mut().push_instruction(
                        Instruction::Add(zinc_types::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRange { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Add(zinc_types::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::IndexRangeInclusive { start, access, .. } => {
                    IntegerConstant::new(
                        start * BigInt::from(access.element_size),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Add(zinc_types::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::StackField { access } => {
                    IntegerConstant::new(
                        BigInt::from(access.offset),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_to_zinc_vm(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::Add(zinc_types::Add),
                        Some(self.identifier.location),
                    );
                }
                SemanticPlaceElement::ContractField { access: _ } => {}
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
