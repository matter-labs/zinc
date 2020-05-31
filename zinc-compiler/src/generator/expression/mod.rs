//!
//! The generator expression.
//!

pub mod element;
pub mod operand;
pub mod operator;

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_bytecode::BuiltinIdentifier;
use zinc_bytecode::DataType;
use zinc_bytecode::Instruction;
use zinc_bytecode::ScalarType;

use crate::generator::bytecode::Bytecode;
use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::operand::place::Place;
use crate::lexical::token::location::Location;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;

use self::element::Element;
use self::operand::Operand;
use self::operator::Operator;

///
/// The expression translated to the target Zinc VM bytecode.
///
#[derive(Debug, Default, Clone)]
pub struct Expression {
    elements: Vec<Element>,
}

impl Expression {
    const ELEMENTS_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY),
        }
    }

    pub fn push_element(&mut self, element: Element) {
        self.elements.push(element)
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.elements.push(Element::Operand(operand))
    }

    pub fn push_operator(&mut self, location: Location, operator: Operator) {
        self.elements.push(Element::Operator { location, operator })
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        for element in self.elements.into_iter() {
            match element {
                Element::Operand(operand) => {
                    operand.write_all_to_bytecode(bytecode.clone());
                }
                Element::Operator { location, operator } => match operator {
                    Operator::None => {}

                    Operator::Assignment { place, expression } => {
                        Self::assignment(bytecode.clone(), place, expression, location)
                    }

                    Operator::AssignmentBitwiseOr {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::BitwiseOr(zinc_bytecode::BitwiseOr),
                        location,
                    ),
                    Operator::AssignmentBitwiseXor {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::BitwiseXor(zinc_bytecode::BitwiseXor),
                        location,
                    ),
                    Operator::AssignmentBitwiseAnd {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::BitwiseAnd(zinc_bytecode::BitwiseAnd),
                        location,
                    ),
                    Operator::AssignmentBitwiseShiftLeft {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::BitwiseShiftLeft(zinc_bytecode::BitwiseShiftLeft),
                        location,
                    ),
                    Operator::AssignmentBitwiseShiftRight {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::BitwiseShiftRight(zinc_bytecode::BitwiseShiftRight),
                        location,
                    ),
                    Operator::AssignmentAddition {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::Add(zinc_bytecode::Add),
                        location,
                    ),
                    Operator::AssignmentSubtraction {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::Sub(zinc_bytecode::Sub),
                        location,
                    ),
                    Operator::AssignmentMultiplication {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::Mul(zinc_bytecode::Mul),
                        location,
                    ),
                    Operator::AssignmentDivision {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::Div(zinc_bytecode::Div),
                        location,
                    ),
                    Operator::AssignmentRemainder {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        bytecode.clone(),
                        place,
                        expression,
                        Instruction::Rem(zinc_bytecode::Rem),
                        location,
                    ),

                    Operator::Or => Self::binary(
                        bytecode.clone(),
                        Instruction::Or(zinc_bytecode::Or),
                        location,
                    ),
                    Operator::Xor => Self::binary(
                        bytecode.clone(),
                        Instruction::Xor(zinc_bytecode::Xor),
                        location,
                    ),
                    Operator::And => Self::binary(
                        bytecode.clone(),
                        Instruction::And(zinc_bytecode::And),
                        location,
                    ),

                    Operator::Equals => Self::binary(
                        bytecode.clone(),
                        Instruction::Eq(zinc_bytecode::Eq),
                        location,
                    ),
                    Operator::NotEquals => Self::binary(
                        bytecode.clone(),
                        Instruction::Ne(zinc_bytecode::Ne),
                        location,
                    ),
                    Operator::GreaterEquals => Self::binary(
                        bytecode.clone(),
                        Instruction::Ge(zinc_bytecode::Ge),
                        location,
                    ),
                    Operator::LesserEquals => Self::binary(
                        bytecode.clone(),
                        Instruction::Le(zinc_bytecode::Le),
                        location,
                    ),
                    Operator::Greater => Self::binary(
                        bytecode.clone(),
                        Instruction::Gt(zinc_bytecode::Gt),
                        location,
                    ),
                    Operator::Lesser => Self::binary(
                        bytecode.clone(),
                        Instruction::Lt(zinc_bytecode::Lt),
                        location,
                    ),

                    Operator::BitwiseOr => Self::binary(
                        bytecode.clone(),
                        Instruction::BitwiseOr(zinc_bytecode::BitwiseOr),
                        location,
                    ),
                    Operator::BitwiseXor => Self::binary(
                        bytecode.clone(),
                        Instruction::BitwiseXor(zinc_bytecode::BitwiseXor),
                        location,
                    ),
                    Operator::BitwiseAnd => Self::binary(
                        bytecode.clone(),
                        Instruction::BitwiseAnd(zinc_bytecode::BitwiseAnd),
                        location,
                    ),
                    Operator::BitwiseShiftLeft => Self::binary(
                        bytecode.clone(),
                        Instruction::BitwiseShiftLeft(zinc_bytecode::BitwiseShiftLeft),
                        location,
                    ),
                    Operator::BitwiseShiftRight => Self::binary(
                        bytecode.clone(),
                        Instruction::BitwiseShiftRight(zinc_bytecode::BitwiseShiftRight),
                        location,
                    ),

                    Operator::Addition => Self::binary(
                        bytecode.clone(),
                        Instruction::Add(zinc_bytecode::Add),
                        location,
                    ),
                    Operator::Subtraction => Self::binary(
                        bytecode.clone(),
                        Instruction::Sub(zinc_bytecode::Sub),
                        location,
                    ),
                    Operator::Multiplication => Self::binary(
                        bytecode.clone(),
                        Instruction::Mul(zinc_bytecode::Mul),
                        location,
                    ),
                    Operator::Division => Self::binary(
                        bytecode.clone(),
                        Instruction::Div(zinc_bytecode::Div),
                        location,
                    ),
                    Operator::Remainder => Self::binary(
                        bytecode.clone(),
                        Instruction::Rem(zinc_bytecode::Rem),
                        location,
                    ),

                    Operator::Casting { r#type } => {
                        if let Some(scalar_type) = r#type.into() {
                            Self::unary(
                                bytecode.clone(),
                                Instruction::Cast(zinc_bytecode::Cast::new(scalar_type)),
                                location,
                            )
                        }
                    }

                    Operator::Not => Self::unary(
                        bytecode.clone(),
                        Instruction::Not(zinc_bytecode::Not),
                        location,
                    ),
                    Operator::BitwiseNot => Self::unary(
                        bytecode.clone(),
                        Instruction::BitwiseNot(zinc_bytecode::BitwiseNot),
                        location,
                    ),
                    Operator::Negation => Self::unary(
                        bytecode.clone(),
                        Instruction::Neg(zinc_bytecode::Neg),
                        location,
                    ),

                    Operator::Index { expression, access } => {
                        if let Some(offset) = access.offset {
                            IntegerConstant::new(
                                BigInt::from(offset),
                                false,
                                zinc_const::BITLENGTH_FIELD,
                            )
                            .write_all_to_bytecode(bytecode.clone());
                        } else {
                            expression.write_all_to_bytecode(bytecode.clone());
                            bytecode.borrow_mut().push_instruction(
                                Instruction::Cast(zinc_bytecode::Cast::new(ScalarType::Field)),
                                Some(location),
                            );
                        }
                        bytecode.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_bytecode::Slice::new(
                                access.total_size,
                                access.element_size,
                            )),
                            Some(location),
                        );
                    }
                    Operator::Slice { access } => {
                        IntegerConstant::new(
                            BigInt::from(access.offset),
                            false,
                            zinc_const::BITLENGTH_FIELD,
                        )
                        .write_all_to_bytecode(bytecode.clone());
                        bytecode.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_bytecode::Slice::new(
                                access.total_size,
                                access.element_size,
                            )),
                            Some(location),
                        );
                    }

                    Operator::Call {
                        type_id,
                        input_size,
                    } => Self::call(bytecode.clone(), type_id, input_size, location),
                    Operator::CallDebug {
                        format,
                        argument_types,
                    } => Self::call_debug(
                        bytecode.clone(),
                        format,
                        argument_types
                            .into_iter()
                            .map(|r#type| r#type.into())
                            .collect(),
                        location,
                    ),
                    Operator::CallAssert { message } => {
                        Self::call_assert(bytecode.clone(), message, location)
                    }
                    Operator::CallStandardLibrary {
                        identifier,
                        input_size,
                        output_size,
                    } => Self::call_standard_library(
                        bytecode.clone(),
                        identifier,
                        input_size,
                        output_size,
                        location,
                    ),
                },
            }
        }
    }

    fn assignment(
        bytecode: Rc<RefCell<Bytecode>>,
        mut place: Place,
        expression: Self,
        location: Location,
    ) {
        match place.memory_type {
            MemoryType::Stack => {
                let is_indexed = !place.elements.is_empty();
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = bytecode
                    .borrow()
                    .get_variable_address(place.identifier.name.as_str())
                    .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                if is_indexed {
                    place.write_all_to_bytecode(bytecode.clone());
                }

                expression.write_all_to_bytecode(bytecode.clone());

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Store(zinc_bytecode::Store::new(address, total_size))
                    },
                    Some(location),
                );
            }
            MemoryType::ContractStorage => {
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = bytecode.borrow_mut().define_variable(None, total_size);

                let storage_index = if let Some(SemanticPlaceElement::ContractField {
                    access:
                        ContractFieldAccess {
                            position,
                            element_size,
                        },
                }) = place.elements.first()
                {
                    let position = *position;

                    IntegerConstant::new(
                        BigInt::from(position),
                        false,
                        zinc_const::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::StorageLoad(zinc_bytecode::StorageLoad::new(*element_size)),
                        Some(place.identifier.location),
                    );

                    place.elements.remove(0);

                    position
                } else {
                    panic!(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                };

                let is_indexed = !place.elements.is_empty();

                bytecode.borrow_mut().push_instruction(
                    Instruction::Store(zinc_bytecode::Store::new(address, total_size)),
                    Some(location),
                );

                if is_indexed {
                    place.write_all_to_bytecode(bytecode.clone());
                }

                expression.write_all_to_bytecode(bytecode.clone());

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Store(zinc_bytecode::Store::new(address, total_size))
                    },
                    Some(location),
                );

                bytecode.borrow_mut().push_instruction(
                    Instruction::Load(zinc_bytecode::Load::new(address, total_size)),
                    Some(location),
                );

                IntegerConstant::new(
                    BigInt::from(storage_index),
                    false,
                    zinc_const::BITLENGTH_FIELD,
                )
                .write_all_to_bytecode(bytecode.clone());
                bytecode.borrow_mut().push_instruction(
                    Instruction::StorageStore(zinc_bytecode::StorageStore::new(total_size)),
                    Some(location),
                );
            }
        }
    }

    fn assignment_with_operation(
        bytecode: Rc<RefCell<Bytecode>>,
        mut place: Place,
        expression: Self,
        operation: Instruction,
        location: Location,
    ) {
        match place.memory_type {
            MemoryType::Stack => {
                let is_indexed = !place.elements.is_empty();
                let address = bytecode
                    .borrow()
                    .get_variable_address(place.identifier.name.as_str())
                    .expect(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
                let element_size = place.element_size;
                let total_size = place.total_size;

                if is_indexed {
                    place.write_all_to_bytecode(bytecode.clone());
                    bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Copy(zinc_bytecode::Copy), Some(location));
                }

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::LoadByIndex(zinc_bytecode::LoadByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Load(zinc_bytecode::Load::new(address, total_size))
                    },
                    Some(location),
                );

                expression.write_all_to_bytecode(bytecode.clone());

                bytecode
                    .borrow_mut()
                    .push_instruction(operation, Some(location));

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Store(zinc_bytecode::Store::new(address, total_size))
                    },
                    Some(location),
                );
            }
            MemoryType::ContractStorage => {
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = bytecode.borrow_mut().define_variable(None, total_size);

                let storage_index = if let Some(SemanticPlaceElement::ContractField {
                    access:
                        ContractFieldAccess {
                            position,
                            element_size,
                        },
                }) = place.elements.first()
                {
                    let position = *position;

                    IntegerConstant::new(
                        BigInt::from(position),
                        false,
                        zinc_const::BITLENGTH_FIELD,
                    )
                    .write_all_to_bytecode(bytecode.clone());
                    bytecode.borrow_mut().push_instruction(
                        Instruction::StorageLoad(zinc_bytecode::StorageLoad::new(*element_size)),
                        Some(place.identifier.location),
                    );

                    place.elements.remove(0);

                    position
                } else {
                    panic!(crate::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                };

                let is_indexed = !place.elements.is_empty();

                bytecode.borrow_mut().push_instruction(
                    Instruction::Store(zinc_bytecode::Store::new(address, total_size)),
                    Some(location),
                );

                if is_indexed {
                    place.write_all_to_bytecode(bytecode.clone());
                    bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Copy(zinc_bytecode::Copy), Some(location));
                }

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::LoadByIndex(zinc_bytecode::LoadByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Load(zinc_bytecode::Load::new(address, total_size))
                    },
                    Some(location),
                );

                expression.write_all_to_bytecode(bytecode.clone());

                bytecode
                    .borrow_mut()
                    .push_instruction(operation, Some(location));

                bytecode.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_bytecode::StoreByIndex::new(
                            address,
                            total_size,
                            element_size,
                        ))
                    } else {
                        Instruction::Store(zinc_bytecode::Store::new(address, total_size))
                    },
                    Some(location),
                );

                bytecode.borrow_mut().push_instruction(
                    Instruction::Load(zinc_bytecode::Load::new(address, total_size)),
                    Some(location),
                );

                IntegerConstant::new(
                    BigInt::from(storage_index),
                    false,
                    zinc_const::BITLENGTH_FIELD,
                )
                .write_all_to_bytecode(bytecode.clone());
                bytecode.borrow_mut().push_instruction(
                    Instruction::StorageStore(zinc_bytecode::StorageStore::new(total_size)),
                    Some(location),
                );
            }
        }
    }

    fn binary(bytecode: Rc<RefCell<Bytecode>>, instruction: Instruction, location: Location) {
        bytecode
            .borrow_mut()
            .push_instruction(instruction, Some(location));
    }

    fn unary(bytecode: Rc<RefCell<Bytecode>>, instruction: Instruction, location: Location) {
        bytecode
            .borrow_mut()
            .push_instruction(instruction, Some(location));
    }

    fn call(
        bytecode: Rc<RefCell<Bytecode>>,
        type_id: usize,
        input_size: usize,
        location: Location,
    ) {
        bytecode.borrow_mut().push_instruction(
            Instruction::Call(zinc_bytecode::Call::new(type_id, input_size)),
            Some(location),
        );
    }

    fn call_debug(
        bytecode: Rc<RefCell<Bytecode>>,
        format: String,
        input_types: Vec<DataType>,
        location: Location,
    ) {
        bytecode.borrow_mut().push_instruction(
            Instruction::Dbg(zinc_bytecode::Dbg::new(format, input_types)),
            Some(location),
        );
    }

    fn call_assert(bytecode: Rc<RefCell<Bytecode>>, message: Option<String>, location: Location) {
        bytecode.borrow_mut().push_instruction(
            Instruction::Assert(zinc_bytecode::Assert::new(message)),
            Some(location),
        );
    }

    fn call_standard_library(
        bytecode: Rc<RefCell<Bytecode>>,
        identifier: BuiltinIdentifier,
        input_size: usize,
        output_size: usize,
        location: Location,
    ) {
        bytecode.borrow_mut().push_instruction(
            Instruction::CallStd(zinc_bytecode::CallStd::new(
                identifier,
                input_size,
                output_size,
            )),
            Some(location),
        );
    }
}
