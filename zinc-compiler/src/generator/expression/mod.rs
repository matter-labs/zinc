//!
//! The generator expression.
//!

pub mod element;
pub mod operand;
pub mod operator;

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;
use num::One;
use num::Zero;

use zinc_build::Instruction;
use zinc_build::LibraryFunctionIdentifier;
use zinc_build::ScalarType;
use zinc_build::Type as BuildType;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::expression::operand::place::Place;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;
use zinc_lexical::Location;

use self::element::Element;
use self::operand::Operand;
use self::operator::Operator;

///
/// The expression translated to the target Zinc VM bytecode.
///
#[derive(Debug, Default, Clone)]
pub struct Expression {
    /// The expression elements array.
    elements: Vec<Element>,
}

impl Expression {
    /// The expression element array default capacity.
    const ELEMENTS_INITIAL_CAPACITY: usize = 16;

    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::ELEMENTS_INITIAL_CAPACITY),
        }
    }

    ///
    /// Pushes an element, that is, either an operator or operand to the expression.
    ///
    pub fn push_element(&mut self, element: Element) {
        self.elements.push(element)
    }

    ///
    /// Pushes an operand to the expression.
    ///
    pub fn push_operand(&mut self, operand: Operand) {
        self.elements.push(Element::Operand(operand))
    }

    ///
    /// Pushes an operator to the expression.
    ///
    pub fn push_operator(&mut self, location: Location, operator: Operator) {
        self.elements.push(Element::Operator { location, operator })
    }

    ///
    /// Appends a subexpression to the expression.
    ///
    pub fn append_expression(&mut self, expression: Self) {
        self.elements.extend(expression.elements);
    }

    ///
    /// Translates an assignment operator into the bytecode.
    ///
    fn assignment(
        state: Rc<RefCell<State>>,
        mut place: Place,
        expression: Self,
        location: Location,
    ) {
        match place.memory_type {
            MemoryType::Stack => {
                let is_indexed = !place.elements.is_empty();
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = state
                    .borrow()
                    .get_variable_address(place.identifier.name.as_str())
                    .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                if is_indexed {
                    place.write_all(state.clone());
                }

                expression.write_all(state.clone());

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_build::StoreByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Store(zinc_build::Store::new(address, total_size))
                    },
                    Some(location),
                );
            }
            MemoryType::ContractStorage => {
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = state.borrow_mut().define_variable(None, total_size);

                let storage_index = if let Some(SemanticPlaceElement::ContractField {
                    access:
                        ContractFieldAccess {
                            position,
                            element_size,
                            ..
                        },
                }) = place.elements.first()
                {
                    let position = *position;

                    IntegerConstant::new(
                        BigInt::from(position),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_all(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::StorageLoad(zinc_build::StorageLoad::new(*element_size)),
                        Some(place.identifier.location),
                    );

                    place.elements.remove(0);

                    position
                } else {
                    panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                };

                let is_indexed = !place.elements.is_empty();

                state.borrow_mut().push_instruction(
                    Instruction::Store(zinc_build::Store::new(address, total_size)),
                    Some(location),
                );

                if is_indexed {
                    place.write_all(state.clone());
                }

                expression.write_all(state.clone());

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_build::StoreByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Store(zinc_build::Store::new(address, total_size))
                    },
                    Some(location),
                );

                state.borrow_mut().push_instruction(
                    Instruction::Load(zinc_build::Load::new(address, total_size)),
                    Some(location),
                );

                IntegerConstant::new(
                    BigInt::from(storage_index),
                    false,
                    zinc_const::bitlength::FIELD,
                )
                .write_all(state.clone());
                state.borrow_mut().push_instruction(
                    Instruction::StorageStore(zinc_build::StorageStore::new(total_size)),
                    Some(location),
                );
            }
        }
    }

    ///
    /// Translates a shortcut assignment operator into the bytecode.
    ///
    fn assignment_with_operation(
        state: Rc<RefCell<State>>,
        mut place: Place,
        expression: Self,
        operation: Instruction,
        location: Location,
    ) {
        match place.memory_type {
            MemoryType::Stack => {
                let is_indexed = !place.elements.is_empty();
                let address = state
                    .borrow()
                    .get_variable_address(place.identifier.name.as_str())
                    .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);
                let element_size = place.element_size;
                let total_size = place.total_size;

                if is_indexed {
                    place.write_all(state.clone());
                    state
                        .borrow_mut()
                        .push_instruction(Instruction::Copy(zinc_build::Copy), Some(location));
                }

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::LoadByIndex(zinc_build::LoadByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Load(zinc_build::Load::new(address, total_size))
                    },
                    Some(location),
                );

                expression.write_all(state.clone());

                state
                    .borrow_mut()
                    .push_instruction(operation, Some(location));

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_build::StoreByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Store(zinc_build::Store::new(address, total_size))
                    },
                    Some(location),
                );
            }
            MemoryType::ContractStorage => {
                let element_size = place.element_size;
                let total_size = place.total_size;
                let address = state.borrow_mut().define_variable(None, total_size);

                let storage_index = if let Some(SemanticPlaceElement::ContractField {
                    access:
                        ContractFieldAccess {
                            position,
                            element_size,
                            ..
                        },
                }) = place.elements.first()
                {
                    let position = *position;

                    IntegerConstant::new(
                        BigInt::from(position),
                        false,
                        zinc_const::bitlength::FIELD,
                    )
                    .write_all(state.clone());
                    state.borrow_mut().push_instruction(
                        Instruction::StorageLoad(zinc_build::StorageLoad::new(*element_size)),
                        Some(place.identifier.location),
                    );

                    place.elements.remove(0);

                    position
                } else {
                    panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS)
                };

                let is_indexed = !place.elements.is_empty();

                state.borrow_mut().push_instruction(
                    Instruction::Store(zinc_build::Store::new(address, total_size)),
                    Some(location),
                );

                if is_indexed {
                    place.write_all(state.clone());
                    state
                        .borrow_mut()
                        .push_instruction(Instruction::Copy(zinc_build::Copy), Some(location));
                }

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::LoadByIndex(zinc_build::LoadByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Load(zinc_build::Load::new(address, total_size))
                    },
                    Some(location),
                );

                expression.write_all(state.clone());

                state
                    .borrow_mut()
                    .push_instruction(operation, Some(location));

                state.borrow_mut().push_instruction(
                    if is_indexed {
                        Instruction::StoreByIndex(zinc_build::StoreByIndex::new(
                            address,
                            element_size,
                            total_size,
                        ))
                    } else {
                        Instruction::Store(zinc_build::Store::new(address, total_size))
                    },
                    Some(location),
                );

                state.borrow_mut().push_instruction(
                    Instruction::Load(zinc_build::Load::new(address, total_size)),
                    Some(location),
                );

                IntegerConstant::new(
                    BigInt::from(storage_index),
                    false,
                    zinc_const::bitlength::FIELD,
                )
                .write_all(state.clone());
                state.borrow_mut().push_instruction(
                    Instruction::StorageStore(zinc_build::StorageStore::new(total_size)),
                    Some(location),
                );
            }
        }
    }

    ///
    /// Translates a binary operator into the bytecode.
    ///
    fn binary(state: Rc<RefCell<State>>, instruction: Instruction, location: Location) {
        state
            .borrow_mut()
            .push_instruction(instruction, Some(location));
    }

    ///
    /// Translates an unary operator into the bytecode.
    ///
    fn unary(state: Rc<RefCell<State>>, instruction: Instruction, location: Location) {
        state
            .borrow_mut()
            .push_instruction(instruction, Some(location));
    }

    ///
    /// Translates an ordinar function call into the bytecode.
    ///
    fn call(state: Rc<RefCell<State>>, type_id: usize, input_size: usize, location: Location) {
        state.borrow_mut().push_instruction(
            Instruction::Call(zinc_build::Call::new(type_id, input_size)),
            Some(location),
        );
    }

    ///
    /// Translates a `dbg!(...)` function call into the bytecode.
    ///
    fn call_debug(
        state: Rc<RefCell<State>>,
        format: String,
        input_types: Vec<BuildType>,
        location: Location,
    ) {
        state.borrow_mut().push_instruction(
            Instruction::Dbg(zinc_build::Dbg::new(format, input_types)),
            Some(location),
        );
    }

    ///
    /// Translates an `require(...)` function call into the bytecode.
    ///
    fn call_assert(state: Rc<RefCell<State>>, message: Option<String>, location: Location) {
        state.borrow_mut().push_instruction(
            Instruction::Require(zinc_build::Require::new(message)),
            Some(location),
        );
    }

    ///
    /// Translates a standard library function call into the bytecode.
    ///
    fn call_standard_library(
        state: Rc<RefCell<State>>,
        identifier: LibraryFunctionIdentifier,
        input_size: usize,
        output_size: usize,
        location: Location,
    ) {
        state.borrow_mut().push_instruction(
            Instruction::CallLibrary(zinc_build::CallLibrary::new(
                identifier,
                input_size,
                output_size,
            )),
            Some(location),
        );
    }
}

impl IBytecodeWritable for Expression {
    fn write_all(self, state: Rc<RefCell<State>>) {
        for element in self.elements.into_iter() {
            match element {
                Element::Operand(operand) => {
                    operand.write_all(state.clone());
                }
                Element::Operator { location, operator } => match operator {
                    Operator::None => {}

                    Operator::Assignment { place, expression } => {
                        Self::assignment(state.clone(), place, expression, location)
                    }

                    Operator::AssignmentBitwiseOr {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::BitwiseOr(zinc_build::BitwiseOr),
                        location,
                    ),
                    Operator::AssignmentBitwiseXor {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::BitwiseXor(zinc_build::BitwiseXor),
                        location,
                    ),
                    Operator::AssignmentBitwiseAnd {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::BitwiseAnd(zinc_build::BitwiseAnd),
                        location,
                    ),
                    Operator::AssignmentBitwiseShiftLeft {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::BitwiseShiftLeft(zinc_build::BitwiseShiftLeft),
                        location,
                    ),
                    Operator::AssignmentBitwiseShiftRight {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::BitwiseShiftRight(zinc_build::BitwiseShiftRight),
                        location,
                    ),
                    Operator::AssignmentAddition {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::Add(zinc_build::Add),
                        location,
                    ),
                    Operator::AssignmentSubtraction {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::Sub(zinc_build::Sub),
                        location,
                    ),
                    Operator::AssignmentMultiplication {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::Mul(zinc_build::Mul),
                        location,
                    ),
                    Operator::AssignmentDivision {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::Div(zinc_build::Div),
                        location,
                    ),
                    Operator::AssignmentRemainder {
                        place,
                        expression,
                        operator: _,
                    } => Self::assignment_with_operation(
                        state.clone(),
                        place,
                        expression,
                        Instruction::Rem(zinc_build::Rem),
                        location,
                    ),

                    Operator::Or => {
                        //                        Self::binary(state.clone(), Instruction::Or(zinc_build::Or), location)
                    }
                    Operator::OrShortCircuitStart => {
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::Not(zinc_build::Not), None);
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::If(zinc_build::If), None);
                    }
                    Operator::OrShortCircuitEnd => {
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_build::Else), None);
                        state.borrow_mut().push_instruction(
                            Instruction::Push(zinc_build::Push::new(
                                BigInt::one(),
                                ScalarType::Boolean,
                            )),
                            None,
                        );
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_build::EndIf), None);
                    }
                    Operator::Xor => {
                        Self::binary(state.clone(), Instruction::Xor(zinc_build::Xor), location)
                    }
                    Operator::And => {
                        //                        Self::binary(state.clone(), Instruction::And(zinc_build::And), location)
                    }
                    Operator::AndShortCircuitStart => {
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::If(zinc_build::If), None);
                    }
                    Operator::AndShortCircuitEnd => {
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_build::Else), None);
                        state.borrow_mut().push_instruction(
                            Instruction::Push(zinc_build::Push::new(
                                BigInt::zero(),
                                ScalarType::Boolean,
                            )),
                            None,
                        );
                        state
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_build::EndIf), None);
                    }

                    Operator::Equals { .. } => {
                        Self::binary(state.clone(), Instruction::Eq(zinc_build::Eq), location)
                    }
                    Operator::NotEquals { .. } => {
                        Self::binary(state.clone(), Instruction::Ne(zinc_build::Ne), location)
                    }
                    Operator::GreaterEquals { .. } => {
                        Self::binary(state.clone(), Instruction::Ge(zinc_build::Ge), location)
                    }
                    Operator::LesserEquals { .. } => {
                        Self::binary(state.clone(), Instruction::Le(zinc_build::Le), location)
                    }
                    Operator::Greater { .. } => {
                        Self::binary(state.clone(), Instruction::Gt(zinc_build::Gt), location)
                    }
                    Operator::Lesser { .. } => {
                        Self::binary(state.clone(), Instruction::Lt(zinc_build::Lt), location)
                    }

                    Operator::BitwiseOr { .. } => Self::binary(
                        state.clone(),
                        Instruction::BitwiseOr(zinc_build::BitwiseOr),
                        location,
                    ),
                    Operator::BitwiseXor { .. } => Self::binary(
                        state.clone(),
                        Instruction::BitwiseXor(zinc_build::BitwiseXor),
                        location,
                    ),
                    Operator::BitwiseAnd { .. } => Self::binary(
                        state.clone(),
                        Instruction::BitwiseAnd(zinc_build::BitwiseAnd),
                        location,
                    ),
                    Operator::BitwiseShiftLeft => Self::binary(
                        state.clone(),
                        Instruction::BitwiseShiftLeft(zinc_build::BitwiseShiftLeft),
                        location,
                    ),
                    Operator::BitwiseShiftRight => Self::binary(
                        state.clone(),
                        Instruction::BitwiseShiftRight(zinc_build::BitwiseShiftRight),
                        location,
                    ),

                    Operator::Addition { .. } => {
                        Self::binary(state.clone(), Instruction::Add(zinc_build::Add), location)
                    }
                    Operator::Subtraction { .. } => {
                        Self::binary(state.clone(), Instruction::Sub(zinc_build::Sub), location)
                    }
                    Operator::Multiplication { .. } => {
                        Self::binary(state.clone(), Instruction::Mul(zinc_build::Mul), location)
                    }
                    Operator::Division { .. } => {
                        Self::binary(state.clone(), Instruction::Div(zinc_build::Div), location)
                    }
                    Operator::Remainder { .. } => {
                        Self::binary(state.clone(), Instruction::Rem(zinc_build::Rem), location)
                    }

                    Operator::Casting { r#type } => {
                        if let Some(scalar_type) = r#type.into() {
                            Self::unary(
                                state.clone(),
                                Instruction::Cast(zinc_build::Cast::new(scalar_type)),
                                location,
                            )
                        }
                    }

                    Operator::Not => {
                        Self::unary(state.clone(), Instruction::Not(zinc_build::Not), location)
                    }
                    Operator::BitwiseNot => Self::unary(
                        state.clone(),
                        Instruction::BitwiseNot(zinc_build::BitwiseNot),
                        location,
                    ),
                    Operator::Negation => {
                        Self::unary(state.clone(), Instruction::Neg(zinc_build::Neg), location)
                    }

                    Operator::Index { expression, access } => {
                        if let Some(offset) = access.offset {
                            IntegerConstant::new(
                                BigInt::from(offset),
                                false,
                                zinc_const::bitlength::FIELD,
                            )
                            .write_all(state.clone());
                        } else {
                            expression.write_all(state.clone());
                            state.borrow_mut().push_instruction(
                                Instruction::Cast(zinc_build::Cast::new(ScalarType::Field)),
                                Some(location),
                            );
                            if access.slice_length == 1 {
                                IntegerConstant::new(
                                    BigInt::from(access.element_size),
                                    false,
                                    zinc_const::bitlength::FIELD,
                                )
                                .write_all(state.clone());
                                state.borrow_mut().push_instruction(
                                    Instruction::Mul(zinc_build::Mul),
                                    Some(location),
                                );
                            }
                        }
                        state.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_build::Slice::new(
                                access.element_size * access.slice_length,
                                access.total_size,
                            )),
                            Some(location),
                        );
                    }
                    Operator::Slice { access } => {
                        IntegerConstant::new(
                            BigInt::from(access.offset),
                            false,
                            zinc_const::bitlength::FIELD,
                        )
                        .write_all(state.clone());
                        state.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_build::Slice::new(
                                access.element_size,
                                access.total_size,
                            )),
                            Some(location),
                        );
                    }

                    Operator::Call {
                        type_id,
                        input_size,
                    } => Self::call(state.clone(), type_id, input_size, location),
                    Operator::CallDebug {
                        format,
                        argument_types,
                    } => Self::call_debug(
                        state.clone(),
                        format,
                        argument_types
                            .into_iter()
                            .map(|r#type| r#type.into())
                            .collect(),
                        location,
                    ),
                    Operator::CallRequire { message } => {
                        Self::call_assert(state.clone(), message, location)
                    }
                    Operator::CallLibrary {
                        identifier,
                        input_size,
                        output_size,
                    } => Self::call_standard_library(
                        state.clone(),
                        identifier,
                        input_size,
                        output_size,
                        location,
                    ),
                },
            }
        }
    }
}
