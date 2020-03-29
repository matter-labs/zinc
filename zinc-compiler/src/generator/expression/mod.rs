//!
//! The generator expression.
//!

pub mod element;
pub mod operand;
pub mod operator;

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;

use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::data::types::DataType;
use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
use crate::generator::r#type::Type;
use crate::semantic::element::access::AccessData;

use self::element::Element;
use self::operand::value::Value as ValueOperand;
use self::operand::Operand;
use self::operator::Operator;

#[derive(Debug, Default, Clone)]
pub struct Expression {
    pub elements: Vec<Element>,
    pub stack: Vec<Operand>,
}

impl Expression {
    const VECTOR_ELEMENTS_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self {
            elements: Vec::with_capacity(Self::VECTOR_ELEMENTS_INITIAL_CAPACITY),
            stack: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
        }
    }

    pub fn push_operand(&mut self, operand: Operand) {
        self.elements.push(Element::Operand(operand))
    }

    pub fn push_operator(&mut self, operator: Operator) {
        self.elements.push(Element::Operator(operator))
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let mut stack = self.stack;

        for element in self.elements.into_iter() {
            match element {
                Element::Operand(operand) => {
                    operand.clone().write_all_to_bytecode(bytecode.clone());
                    stack.push(operand);
                }
                Element::Operator(operator) => match operator {
                    Operator::Assignment => Self::assignment(&mut stack, bytecode.clone()),
                    Operator::AssignmentBitwiseOr => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitOr(zinc_bytecode::BitOr),
                    ),
                    Operator::AssignmentBitwiseXor => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitXor(zinc_bytecode::BitXor),
                    ),
                    Operator::AssignmentBitwiseAnd => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitAnd(zinc_bytecode::BitAnd),
                    ),
                    Operator::AssignmentBitwiseShiftLeft => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitShiftLeft(zinc_bytecode::BitShiftLeft),
                    ),
                    Operator::AssignmentBitwiseShiftRight => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitShiftRight(zinc_bytecode::BitShiftRight),
                    ),
                    Operator::AssignmentAddition => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Add(zinc_bytecode::Add),
                    ),
                    Operator::AssignmentSubtraction => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Sub(zinc_bytecode::Sub),
                    ),
                    Operator::AssignmentMultiplication => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Mul(zinc_bytecode::Mul),
                    ),
                    Operator::AssignmentDivision => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Div(zinc_bytecode::Div),
                    ),
                    Operator::AssignmentRemainder => Self::assignment_with_operation(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Rem(zinc_bytecode::Rem),
                    ),

                    Operator::Or => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Or(zinc_bytecode::Or),
                    ),
                    Operator::Xor => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Xor(zinc_bytecode::Xor),
                    ),
                    Operator::And => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::And(zinc_bytecode::And),
                    ),

                    Operator::Equals => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Eq(zinc_bytecode::Eq),
                    ),
                    Operator::NotEquals => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Ne(zinc_bytecode::Ne),
                    ),
                    Operator::GreaterEquals => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Ge(zinc_bytecode::Ge),
                    ),
                    Operator::LesserEquals => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Le(zinc_bytecode::Le),
                    ),
                    Operator::Greater => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Gt(zinc_bytecode::Gt),
                    ),
                    Operator::Lesser => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Lt(zinc_bytecode::Lt),
                    ),

                    Operator::BitwiseOr => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitOr(zinc_bytecode::BitOr),
                    ),
                    Operator::BitwiseXor => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitXor(zinc_bytecode::BitXor),
                    ),
                    Operator::BitwiseAnd => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitAnd(zinc_bytecode::BitAnd),
                    ),
                    Operator::BitwiseShiftLeft => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitShiftLeft(zinc_bytecode::BitShiftLeft),
                    ),
                    Operator::BitwiseShiftRight => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitShiftRight(zinc_bytecode::BitShiftRight),
                    ),

                    Operator::Addition => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Add(zinc_bytecode::Add),
                    ),
                    Operator::Subtraction => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Sub(zinc_bytecode::Sub),
                    ),
                    Operator::Multiplication => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Mul(zinc_bytecode::Mul),
                    ),
                    Operator::Division => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Div(zinc_bytecode::Div),
                    ),
                    Operator::Remainder => Self::binary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Rem(zinc_bytecode::Rem),
                    ),

                    Operator::Casting { r#type } => {
                        if let Some(scalar_type) = r#type.into() {
                            Self::unary(
                                &mut stack,
                                bytecode.clone(),
                                Instruction::Cast(zinc_bytecode::Cast::new(scalar_type)),
                            )
                        }
                    }

                    Operator::Not => Self::unary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Not(zinc_bytecode::Not),
                    ),
                    Operator::BitwiseNot => Self::unary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::BitNot(zinc_bytecode::BitNot),
                    ),
                    Operator::Negation => Self::unary(
                        &mut stack,
                        bytecode.clone(),
                        Instruction::Neg(zinc_bytecode::Neg),
                    ),

                    Operator::Index { data } => Self::index(&mut stack, bytecode.clone(), data),
                    Operator::Slice { data } => Self::slice(&mut stack, bytecode.clone(), data),

                    Operator::Call {
                        unique_id,
                        input_size,
                    } => Self::call(&mut stack, bytecode.clone(), unique_id, input_size),
                    Operator::CallDebug {
                        format,
                        argument_types,
                    } => Self::call_debug(
                        &mut stack,
                        bytecode.clone(),
                        format,
                        argument_types
                            .into_iter()
                            .map(|r#type| r#type.into())
                            .collect(),
                    ),
                    Operator::CallAssert { message } => {
                        Self::call_assert(&mut stack, bytecode.clone(), message)
                    }
                    Operator::CallStandardLibrary {
                        identifier,
                        input_size,
                        output_size,
                    } => Self::call_standard_library(
                        &mut stack,
                        bytecode.clone(),
                        identifier,
                        input_size,
                        output_size,
                    ),
                },
            }
        }
    }

    fn assignment(stack: &mut Vec<Operand>, bytecode: Rc<RefCell<Bytecode>>) {
        let _operand_2 = stack.pop().unwrap();
        let operand_1 = stack.pop().unwrap();

        let address = match operand_1 {
            Operand::Variable(variable) => bytecode
                .borrow()
                .get_variable_address(variable.name.as_str())
                .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
            Operand::Memory(memory) => memory.address,
            _ => panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::Store(zinc_bytecode::Store::new(address)),
            crate::lexical::Location::default(),
        );
    }

    fn assignment_with_operation(
        stack: &mut Vec<Operand>,
        bytecode: Rc<RefCell<Bytecode>>,
        instruction: Instruction,
    ) {
        let _operand_2 = stack.pop().unwrap();
        let operand_1 = stack.pop().unwrap();

        let address = match operand_1 {
            Operand::Variable(variable) => bytecode
                .borrow()
                .get_variable_address(variable.name.as_str())
                .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
            Operand::Memory(memory) => memory.address,
            _ => panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::Load(zinc_bytecode::Load::new(address)),
            crate::lexical::Location::default(),
        );
        bytecode
            .borrow_mut()
            .push_instruction(instruction, crate::lexical::Location::default());
        bytecode.borrow_mut().push_instruction(
            Instruction::Store(zinc_bytecode::Store::new(address)),
            crate::lexical::Location::default(),
        );
    }

    fn binary(stack: &mut Vec<Operand>, bytecode: Rc<RefCell<Bytecode>>, instruction: Instruction) {
        let _operand_2 = stack.pop().unwrap();
        let operand_1 = stack.pop().unwrap();

        stack.push(operand_1.clone());

        bytecode
            .borrow_mut()
            .push_instruction(instruction, crate::lexical::Location::default());
    }

    fn unary(stack: &mut Vec<Operand>, bytecode: Rc<RefCell<Bytecode>>, instruction: Instruction) {
        let operand = stack.pop().unwrap();

        stack.push(operand.clone());

        bytecode
            .borrow_mut()
            .push_instruction(instruction, crate::lexical::Location::default());
    }

    fn index(stack: &mut Vec<Operand>, bytecode: Rc<RefCell<Bytecode>>, data: AccessData) {
        let _operand_2 = stack.pop().unwrap();
        let operand_1 = stack.pop().unwrap();

        match operand_1 {
            Operand::Value(ValueOperand {
                r#type: Type::Array { .. },
            })
            | Operand::Array(_) => {
                if let Some(r#type) = Type::try_from_semantic(&data.sliced_type) {
                    stack.push(Operand::Value(ValueOperand::new(r#type)));
                }
                bytecode.borrow_mut().push_instruction(
                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                        BigInt::from(data.sliced_type.size()),
                        ScalarType::Field,
                    )),
                    crate::lexical::Location::default(),
                );
                bytecode.borrow_mut().push_instruction(
                    Instruction::Mul(zinc_bytecode::Mul),
                    crate::lexical::Location::default(),
                );
                bytecode.borrow_mut().push_instruction(
                    Instruction::Slice(zinc_bytecode::Slice::new(
                        data.total_size,
                        data.element_size,
                    )),
                    crate::lexical::Location::default(),
                );
            }
            // Operand::Variable(structure) => {},
            _ => panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
        }
    }

    fn slice(stack: &mut Vec<Operand>, bytecode: Rc<RefCell<Bytecode>>, data: AccessData) {
        let operand = stack.pop().unwrap();

        match operand {
            Operand::Value(ValueOperand {
                r#type: Type::Tuple { .. },
            })
            | Operand::Tuple(_)
            | Operand::Value(ValueOperand {
                r#type: Type::Structure { .. },
            })
            | Operand::Structure(_) => {
                if let Some(r#type) = Type::try_from_semantic(&data.sliced_type) {
                    stack.push(Operand::Value(ValueOperand::new(r#type)));
                }
                bytecode.borrow_mut().push_instruction(
                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                        BigInt::from(data.offset),
                        ScalarType::Field,
                    )),
                    crate::lexical::Location::default(),
                );
                bytecode.borrow_mut().push_instruction(
                    Instruction::Slice(zinc_bytecode::Slice::new(
                        data.total_size,
                        data.element_size,
                    )),
                    crate::lexical::Location::default(),
                );
            }
            // Operand::Variable(structure) => {},
            _ => panic!(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS),
        }
    }

    fn call(
        stack: &mut Vec<Operand>,
        bytecode: Rc<RefCell<Bytecode>>,
        unique_id: usize,
        input_size: usize,
    ) {
        let operand = stack.pop().unwrap();

        stack.push(operand.clone());

        let address = bytecode
            .borrow()
            .get_function_address(unique_id)
            .expect(crate::generator::PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS);

        bytecode.borrow_mut().push_instruction(
            Instruction::Call(zinc_bytecode::Call::new(address, input_size)),
            crate::lexical::Location::default(),
        );
    }

    fn call_debug(
        stack: &mut Vec<Operand>,
        bytecode: Rc<RefCell<Bytecode>>,
        format: String,
        input_types: Vec<DataType>,
    ) {
        let operand = stack.pop().unwrap();

        stack.push(operand.clone());

        bytecode.borrow_mut().push_instruction(
            Instruction::Dbg(zinc_bytecode::Dbg::new(format, input_types)),
            crate::lexical::Location::default(),
        );
    }

    fn call_assert(
        stack: &mut Vec<Operand>,
        bytecode: Rc<RefCell<Bytecode>>,
        message: Option<String>,
    ) {
        let operand = stack.pop().unwrap();

        stack.push(operand.clone());

        bytecode.borrow_mut().push_instruction(
            Instruction::Assert(zinc_bytecode::Assert::new(message)),
            crate::lexical::Location::default(),
        );
    }

    fn call_standard_library(
        stack: &mut Vec<Operand>,
        bytecode: Rc<RefCell<Bytecode>>,
        identifier: BuiltinIdentifier,
        input_size: usize,
        output_size: usize,
    ) {
        let operand = stack.pop().unwrap();

        stack.push(operand.clone());

        bytecode.borrow_mut().push_instruction(
            Instruction::CallBuiltin(zinc_bytecode::CallBuiltin::new(
                identifier,
                input_size,
                output_size,
            )),
            crate::lexical::Location::default(),
        );
    }
}
