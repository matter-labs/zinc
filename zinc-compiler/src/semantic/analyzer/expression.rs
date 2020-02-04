//!
//! The expression semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use zinc_bytecode::Instruction;

use crate::semantic::Array;
use crate::semantic::Bytecode;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::FunctionType;
use crate::semantic::IntegerConstant;
use crate::semantic::Path;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::ScopeVariableItem;
use crate::semantic::StandardLibraryFunctionType;
use crate::semantic::StatementAnalyzer;
use crate::semantic::Structure;
use crate::semantic::StructureValueError;
use crate::semantic::TranslationHint;
use crate::semantic::Tuple;
use crate::semantic::Type;
use crate::semantic::Value;
use crate::syntax;
use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionAuxiliary;
use crate::syntax::ExpressionElement;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpression;
use crate::syntax::MatchPatternVariant;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    bytecode: Rc<RefCell<Bytecode>>,

    is_next_call_instruction: bool,
    operands: Vec<StackElement>,
}

#[derive(Debug, Clone)]
enum StackElement {
    NotEvaluated(ExpressionOperand),
    Evaluated(Element),
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;

    pub fn new(scope: Rc<RefCell<Scope>>, bytecode: Rc<RefCell<Bytecode>>) -> Self {
        Self {
            scope_stack: {
                let mut scope_stack = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scope_stack.push(scope);
                scope_stack
            },
            bytecode,

            is_next_call_instruction: false,
            operands: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
        }
    }

    pub fn new_without_bytecode(scope: Rc<RefCell<Scope>>) -> Self {
        Self::new(scope, Rc::new(RefCell::new(Bytecode::default())))
    }

    pub fn expression(
        &mut self,
        expression: Expression,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = expression.location;
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    let place = operand_1
                        .assign(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    if !place.is_mutable {
                        return Err(Error::AssignmentToImmutableMemory(
                            location,
                            place.to_string(),
                        ));
                    }
                    if place.r#type != r#type {
                        return Err(Error::AssignmentTypesMismatch(
                            location,
                            r#type.to_string(),
                            place.r#type.to_string(),
                        ));
                    }

                    self.bytecode.borrow_mut().push_instruction_store(
                        place.address,
                        place.r#type.size(),
                        if place.is_indexed {
                            Some(place.total_size)
                        } else {
                            None
                        },
                        false,
                        element.location,
                    );
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode.borrow_mut().push_instruction(
                        Instruction::Pop(zinc_bytecode::Pop::new(2)),
                        element.location,
                    );

                    let result = operand_1
                        .range_inclusive(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode.borrow_mut().push_instruction(
                        Instruction::Pop(zinc_bytecode::Pop::new(2)),
                        element.location,
                    );

                    let result = operand_1
                        .range(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Or(zinc_bytecode::Or), element.location);

                    let result = operand_1
                        .or(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Xor(zinc_bytecode::Xor), element.location);

                    let result = operand_1
                        .xor(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::And(zinc_bytecode::And), element.location);

                    let result = operand_1
                        .and(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), element.location);

                    let result = operand_1
                        .equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ne(zinc_bytecode::Ne), element.location);

                    let result = operand_1
                        .not_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ge(zinc_bytecode::Ge), element.location);

                    let result = operand_1
                        .greater_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Le(zinc_bytecode::Le), element.location);

                    let result = operand_1
                        .lesser_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Gt(zinc_bytecode::Gt), element.location);

                    let result = operand_1
                        .greater(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Lt(zinc_bytecode::Lt), element.location);

                    let result = operand_1
                        .lesser(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Add(zinc_bytecode::Add), element.location);

                    let result = operand_1
                        .add(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Sub(zinc_bytecode::Sub), element.location);

                    let result = operand_1
                        .subtract(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Mul(zinc_bytecode::Mul), element.location);

                    let result = operand_1
                        .multiply(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Div(zinc_bytecode::Div), element.location);

                    let result = operand_1
                        .divide(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Rem(zinc_bytecode::Rem), element.location);

                    let result = operand_1
                        .remainder(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::ValueExpression,
                        TranslationHint::TypeExpression,
                    )?;

                    if let Some((is_signed, bitlength)) = operand_1
                        .cast(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?
                    {
                        self.bytecode.borrow_mut().push_instruction(
                            Instruction::Cast(zinc_bytecode::Cast::new(is_signed, bitlength)),
                            element.location,
                        );
                    }

                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Neg(zinc_bytecode::Neg), element.location);

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 =
                        self.evaluate_unary_operand(TranslationHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Not(zinc_bytecode::Not), element.location);

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Reference) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Dereference) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Index) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::ValueExpression,
                    )?;

                    let is_place_indexed = match operand_1 {
                        Element::Place(ref place) => place.is_indexed,
                        _ => false,
                    };

                    let result = operand_1
                        .index(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    match operand_1 {
                        operand @ Element::Place(_) => {
                            if let Element::Constant(Constant::Range(_))
                            | Element::Constant(Constant::RangeInclusive(_)) = operand_2
                            {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::from(result.offset),
                                        false,
                                        crate::BITLENGTH_FIELD,
                                    )),
                                    element.location,
                                );
                            } else {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Cast(zinc_bytecode::Cast::new(
                                        false,
                                        crate::BITLENGTH_FIELD,
                                    )),
                                    element.location,
                                );
                            }
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::zero(),
                                        false,
                                        crate::BITLENGTH_FIELD,
                                    )),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.element_size),
                                    false,
                                    crate::BITLENGTH_FIELD,
                                )),
                                element.location,
                            );
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Add(zinc_bytecode::Add),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Mul(zinc_bytecode::Mul),
                                element.location,
                            );
                            if is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::Add(zinc_bytecode::Add),
                                    element.location,
                                );
                            }
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            match operand_2 {
                                Element::Constant(Constant::Range(_))
                                | Element::Constant(Constant::RangeInclusive(_)) => {
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::PushConst(zinc_bytecode::PushConst::new(
                                            BigInt::from(result.offset),
                                            false,
                                            crate::BITLENGTH_FIELD,
                                        )),
                                        element.location,
                                    );
                                }
                                _ => {
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::Cast(zinc_bytecode::Cast::new(
                                            false,
                                            crate::BITLENGTH_FIELD,
                                        )),
                                        element.location,
                                    );
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::PushConst(zinc_bytecode::PushConst::new(
                                            BigInt::from(result.element_size),
                                            false,
                                            crate::BITLENGTH_FIELD,
                                        )),
                                        element.location,
                                    );
                                    self.bytecode.borrow_mut().push_instruction(
                                        Instruction::Mul(zinc_bytecode::Mul),
                                        element.location,
                                    );
                                }
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Slice(zinc_bytecode::Slice::new(
                                    result.total_size,
                                    result.element_size,
                                )),
                                element.location,
                            );
                            self.push_operand(StackElement::Evaluated(Element::Value(
                                result.sliced_value.expect(
                                    crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SLICED_VALUE,
                                ),
                            )));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PlaceExpression,
                        TranslationHint::CompoundTypeMember,
                    )?;

                    let is_place_indexed = match operand_1 {
                        Element::Place(ref place) => place.is_indexed,
                        _ => false,
                    };

                    let result = operand_1
                        .field(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;

                    match operand_1 {
                        operand @ Element::Place(_) => {
                            if !is_place_indexed {
                                self.bytecode.borrow_mut().push_instruction(
                                    Instruction::PushConst(zinc_bytecode::PushConst::new(
                                        BigInt::zero(),
                                        false,
                                        crate::BITLENGTH_FIELD,
                                    )),
                                    element.location,
                                );
                            }
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.offset),
                                    false,
                                    crate::BITLENGTH_FIELD,
                                )),
                                element.location,
                            );
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Add(zinc_bytecode::Add),
                                element.location,
                            );
                            self.push_operand(StackElement::Evaluated(operand));
                        }
                        Element::Value(_) | Element::Constant(_) => {
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::PushConst(zinc_bytecode::PushConst::new(
                                    BigInt::from(result.offset),
                                    false,
                                    crate::BITLENGTH_FIELD,
                                )),
                                element.location,
                            );
                            self.bytecode.borrow_mut().push_instruction(
                                Instruction::Slice(zinc_bytecode::Slice::new(
                                    result.total_size,
                                    result.element_size,
                                )),
                                element.location,
                            );
                            self.push_operand(StackElement::Evaluated(Element::Value(
                                result.sliced_value.expect(
                                    crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SLICED_VALUE,
                                ),
                            )));
                        }
                        _ => {}
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    self.operator_function_call(element)?
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        TranslationHint::PathExpression,
                        TranslationHint::CompoundTypeMember,
                    )?;

                    operand_1
                        .path(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::Instruction) => {
                    self.is_next_call_instruction = true;
                }
                ExpressionObject::Auxiliary(ExpressionAuxiliary::PlaceEnd) => {
                    let element = self
                        .evaluate_operand(TranslationHint::ValueExpression)
                        .map(StackElement::Evaluated)?;
                    self.push_operand(element);
                }
            }
        }

        self.evaluate_operand(translation_hint)
    }

    pub fn operator_function_call(&mut self, element: ExpressionElement) -> Result<(), Error> {
        let (operand_1, operand_2) = self.evaluate_binary_operands(
            TranslationHint::TypeExpression,
            TranslationHint::ValueExpression,
        )?;

        let function = match operand_1 {
            Element::Type(Type::Function(function)) => function,
            Element::Path(path) => match Scope::resolve_path(self.scope(), &path)? {
                ScopeItem::Type(Type::Function(function)) => function,
                item => {
                    return Err(Error::FunctionCallingNotCallableObject(
                        element.location,
                        item.to_string(),
                    ))
                }
            },
            operand => {
                return Err(Error::FunctionCallingNotCallableObject(
                    element.location,
                    operand.to_string(),
                ))
            }
        };

        // check the number of the arguments
        let argument_elements = match operand_2 {
            Element::ArgumentList(values) => values,
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
        };

        let return_type = match function {
            FunctionType::UserDefined(function) => {
                if self.is_next_call_instruction {
                    return Err(Error::FunctionInstructionUnknown(
                        element.location,
                        function.identifier,
                    ));
                }

                if argument_elements.len() != function.arguments.len() {
                    return Err(Error::FunctionArgumentCountMismatch(
                        element.location,
                        function.identifier,
                        function.arguments.len(),
                        argument_elements.len(),
                    ));
                }

                let function_address = self
                    .bytecode
                    .borrow_mut()
                    .function_address(function.identifier.as_str())
                    .expect(crate::semantic::PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS);
                let function_input_size = function
                    .arguments
                    .iter()
                    .map(|(_name, r#type)| r#type.size())
                    .sum();

                for (argument_index, (argument_name, expected_type)) in
                    function.arguments.into_iter().enumerate()
                {
                    let actual_type =
                        Type::from_element(&argument_elements[argument_index], self.scope())?;
                    if expected_type != actual_type {
                        return Err(Error::FunctionArgumentTypeMismatch(
                            element.location,
                            function.identifier,
                            argument_name,
                            expected_type.to_string(),
                            actual_type.to_string(),
                        ));
                    }
                }

                self.bytecode.borrow_mut().push_data_stack_address();
                self.bytecode.borrow_mut().push_instruction(
                    Instruction::Call(zinc_bytecode::Call::new(
                        function_address,
                        function_input_size,
                    )),
                    element.location,
                );
                self.bytecode.borrow_mut().pop_data_stack_address();

                *function.return_type
            }
            FunctionType::DebugInstruction(instruction) => {
                if !self.is_next_call_instruction {
                    return Err(Error::FunctionInstructionSpecifierMissing(
                        element.location,
                        instruction.identifier,
                    ));
                }

                let string = match argument_elements.get(0) {
                    Some(Element::Constant(Constant::String(string))) => string.to_owned(),
                    Some(argument) => {
                        return Err(Error::InstructionDebugExpectedString(
                            element.location,
                            argument.to_string(),
                        ))
                    }
                    None => {
                        return Err(Error::InstructionDebugExpectedString(
                            element.location,
                            "None".to_owned(),
                        ))
                    }
                };

                let debug_input_size = argument_elements
                    .into_iter()
                    .skip(1)
                    .map(|argument| match argument {
                        Element::Constant(constant) => constant.r#type().size(),
                        Element::Value(value) => value.r#type().size(),
                        _ => 0,
                    })
                    .sum();

                self.bytecode.borrow_mut().push_instruction(
                    Instruction::Log(zinc_bytecode::Dbg::new(string, debug_input_size)),
                    element.location,
                );

                Type::new_unit()
            }
            FunctionType::AssertInstruction(instruction) => {
                if !self.is_next_call_instruction {
                    return Err(Error::FunctionInstructionSpecifierMissing(
                        element.location,
                        instruction.identifier,
                    ));
                }

                match argument_elements.get(0) {
                    Some(Element::Constant(Constant::Boolean(_))) => {}
                    Some(Element::Value(Value::Boolean)) => {}
                    Some(argument) => {
                        return Err(Error::InstructionAssertExpectedBoolean(
                            element.location,
                            argument.to_string(),
                        ))
                    }
                    None => {
                        return Err(Error::InstructionAssertExpectedBoolean(
                            element.location,
                            "None".to_owned(),
                        ))
                    }
                }

                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::Assert(zinc_bytecode::Assert), element.location);

                Type::new_unit()
            }
            FunctionType::StandardLibrary(function) => {
                if self.is_next_call_instruction {
                    return Err(Error::FunctionInstructionUnknown(
                        element.location,
                        function.identifier().to_owned(),
                    ));
                }

                let builtin_identifier = function.builtin_identifier();

                let mut arguments = Vec::with_capacity(argument_elements.len());
                for element in argument_elements.iter() {
                    arguments.push(Type::from_element(element, self.scope())?);
                }

                let return_type = match function {
                    StandardLibraryFunctionType::Sha256(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::Pedersen(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::ToBits(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::FromBitsUnsigned(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::FromBitsSigned(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::FromBitsField(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::ArrayReverse(function) => function
                        .validate(arguments.as_slice())
                        .map_err(|error| Error::FunctionStandardLibrary(element.location, error))?,
                    StandardLibraryFunctionType::ArrayTruncate(function) => {
                        match argument_elements.get(1) {
                            Some(Element::Constant(Constant::Integer(
                                integer @ IntegerConstant { .. },
                            ))) if !integer.is_signed => {
                                let new_length = integer.to_usize().unwrap();
                                function
                                    .validate(arguments.as_slice(), new_length)
                                    .map_err(|error| {
                                        Error::FunctionStandardLibrary(element.location, error)
                                    })?
                            }
                            argument => {
                                return Err(Error::FunctionExpectedConstantLengthArgument(
                                    element.location,
                                    function.identifier,
                                    format!("{:?}", argument),
                                ))
                            }
                        }
                    }
                    StandardLibraryFunctionType::ArrayPad(function) => {
                        match argument_elements.get(1) {
                            Some(Element::Constant(Constant::Integer(
                                integer @ IntegerConstant { .. },
                            ))) if !integer.is_signed => {
                                let new_length = integer.to_usize().unwrap();
                                function
                                    .validate(arguments.as_slice(), new_length)
                                    .map_err(|error| {
                                        Error::FunctionStandardLibrary(element.location, error)
                                    })?
                            }
                            argument => {
                                return Err(Error::FunctionExpectedConstantLengthArgument(
                                    element.location,
                                    function.identifier,
                                    format!("{:?}", argument),
                                ))
                            }
                        }
                    }
                };

                self.bytecode.borrow_mut().push_instruction(
                    Instruction::CallBuiltin(zinc_bytecode::CallBuiltin::new(
                        builtin_identifier,
                        arguments.into_iter().map(|r#type| r#type.size()).sum(),
                        return_type.size(),
                    )),
                    element.location,
                );

                return_type
            }
        };

        self.is_next_call_instruction = false;
        self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
            return_type,
        ))));
        Ok(())
    }

    pub fn block_expression(&mut self, block: BlockExpression) -> Result<Element, Error> {
        for statement in block.statements.into_iter() {
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), HashMap::new())
                .function_local_statement(statement)?;
        }
        match block.expression {
            Some(expression) => self.expression(*expression, TranslationHint::ValueExpression),
            None => Ok(Element::Value(Value::Unit)),
        }
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let constant = Constant::from(literal);
        self.bytecode
            .borrow_mut()
            .push_instruction(constant.to_instruction(), location);
        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let integer = IntegerConstant::try_from(&literal)
            .map_err(|error| Error::InferenceConstant(location, error))?;
        self.bytecode
            .borrow_mut()
            .push_instruction(integer.to_instruction(), location);
        Ok(Element::Constant(Constant::Integer(integer)))
    }

    fn string_literal(&mut self, literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }

    fn member_integer(&mut self, integer: MemberInteger) -> Result<Element, Error> {
        let location = integer.location;
        let integer = IntegerConstant::try_from(&integer.literal)
            .map_err(|error| Error::InferenceConstant(location, error))?
            .to_usize()
            .map_err(|error| Error::InferenceConstant(location, error))?;
        Ok(Element::MemberInteger(integer))
    }

    fn member_string(&mut self, member_string: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_string))
    }

    fn identifier(
        &mut self,
        identifier: Identifier,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = identifier.location;

        let path = Path::new(location, identifier.into());
        self.translate_path(&path, translation_hint)
    }

    fn r#type(&mut self, r#type: syntax::Type) -> Result<Element, Error> {
        Type::from_type_variant(&r#type.variant, self.scope()).map(Element::Type)
    }

    fn conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        // compile the condition and check if it is boolean
        let condition_result =
            self.expression(*conditional.condition, TranslationHint::ValueExpression)?;
        match Type::from_element(&condition_result, self.scope())? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition(
                    condition_location,
                    r#type.to_string(),
                ))
            }
        }

        self.bytecode
            .borrow_mut()
            .push_instruction(Instruction::If(zinc_bytecode::If), location);

        self.push_scope();
        let main_result = self.block_expression(conditional.main_block)?;
        let main_type = Type::from_element(&main_result, self.scope())?;
        self.pop_scope();

        let else_type = if let Some(else_block) = conditional.else_block {
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_bytecode::Else), else_block.location);

            self.push_scope();
            let else_result = self.block_expression(else_block)?;
            let else_type = Type::from_element(&else_result, self.scope())?;
            self.pop_scope();

            else_type
        } else {
            Type::Unit
        };

        self.bytecode
            .borrow_mut()
            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch(
                location,
                main_type.to_string(),
                else_type.to_string(),
            ));
        }

        Ok(main_result)
    }

    fn match_expression(&mut self, r#match: MatchExpression) -> Result<Element, Error> {
        let location = r#match.location;

        let scrutinee_location = r#match.scrutinee.location;
        let scrutinee_result =
            self.expression(r#match.scrutinee, TranslationHint::ValueExpression)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, self.scope())?;
        let scrutinee_size = scrutinee_type.size();
        let scrutinee_address = self
            .bytecode
            .borrow_mut()
            .allocate_data_stack_space(scrutinee_size);
        self.bytecode.borrow_mut().push_instruction_store(
            scrutinee_address,
            scrutinee_size,
            None,
            false,
            scrutinee_location,
        );

        let mut is_exhausted = false;
        let mut branch_results = Vec::with_capacity(r#match.branches.len());
        let mut endifs = 0;
        for (index, (pattern, expression)) in r#match.branches.into_iter().enumerate() {
            let pattern_location = pattern.location;
            let expression_location = expression.location;

            if is_exhausted {
                return Err(Error::MatchBranchUnreachable(pattern.location));
            }
            let result = match pattern.variant {
                MatchPatternVariant::BooleanLiteral(boolean) => {
                    let location = boolean.location;

                    let constant = Constant::from(boolean);
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType(
                            pattern_location,
                            pattern_type.to_string(),
                            scrutinee_type.to_string(),
                        ));
                    }

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(constant.to_instruction(), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::IntegerLiteral(integer) => {
                    let location = integer.location;

                    let constant = IntegerConstant::try_from(&integer)
                        .map_err(|error| Error::InferenceConstant(location, error))?;
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType(
                            pattern_location,
                            pattern_type.to_string(),
                            scrutinee_type.to_string(),
                        ));
                    }

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(constant.to_instruction(), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                    }
                    self.push_scope();
                    self.scope()
                        .borrow_mut()
                        .declare_variable(
                            identifier.name,
                            ScopeVariableItem::new(
                                scrutinee_type.clone(),
                                false,
                                scrutinee_address,
                            ),
                        )
                        .map_err(|error| Error::Scope(location, error))?;
                    let result = self.expression(expression, TranslationHint::ValueExpression)?;
                    self.pop_scope();

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
                    }

                    result
                }
                MatchPatternVariant::Path(path) => {
                    let location = path.location;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                        endifs += 1;
                    }

                    let path = match self.expression(path, TranslationHint::PathExpression)? {
                        Element::Path(path) => path,
                        element => {
                            return Err(Error::MatchBranchPatternPathExpectedEvaluable(
                                location,
                                element.to_string(),
                            ))
                        }
                    };

                    self.bytecode.borrow_mut().push_instruction_load(
                        scrutinee_address,
                        scrutinee_size,
                        None,
                        false,
                        scrutinee_location,
                    );
                    match Scope::resolve_path(self.scope(), &path)? {
                        ScopeItem::Variable(variable) => {
                            self.bytecode.borrow_mut().push_instruction_load(
                                variable.address,
                                variable.r#type.size(),
                                None,
                                false,
                                location,
                            )
                        }
                        ScopeItem::Static(r#static) => {
                            self.bytecode.borrow_mut().push_instruction_load(
                                r#static.address,
                                r#static.data.r#type().size(),
                                None,
                                true,
                                location,
                            )
                        }
                        ScopeItem::Constant(constant) => self
                            .bytecode
                            .borrow_mut()
                            .push_instruction(constant.to_instruction(), location),
                        item => {
                            return Err(Error::MatchBranchPatternPathExpectedEvaluable(
                                path.location,
                                item.to_string(),
                            ))
                        }
                    }
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq), location);
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If), location);

                    self.expression(expression, TranslationHint::ValueExpression)?
                }
                MatchPatternVariant::Wildcard => {
                    let location = expression.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else), location);
                    }

                    let result = self.expression(expression, TranslationHint::ValueExpression)?;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
                    }

                    result
                }
            };

            let result_type = Type::from_element(&result, self.scope())?;
            if let Some(first_branch_result) = branch_results.get(0) {
                let first_branch_result_type =
                    Type::from_element(first_branch_result, self.scope())?;
                if result_type != first_branch_result_type {
                    return Err(Error::MatchBranchExpressionInvalidType(
                        expression_location,
                        result_type.to_string(),
                        first_branch_result_type.to_string(),
                    ));
                }
            }

            branch_results.push(result);
        }

        for _ in 0..endifs {
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        }

        if !is_exhausted {
            return Err(Error::MatchNotExhausted(location));
        }

        Ok(scrutinee_result)
    }

    fn array_expression(&mut self, array: ArrayExpression) -> Result<Element, Error> {
        let mut result = Array::default();

        for expression in array.elements.into_iter() {
            let location = expression.location;
            match array.repeats_count {
                Some(ref repeats_count) => {
                    let repeats_count = IntegerConstant::try_from(repeats_count)
                        .map_err(|error| Error::InferenceConstant(location, error))?
                        .to_usize()
                        .map_err(|error| Error::InferenceConstant(location, error))?;
                    for _ in 0..repeats_count {
                        let element =
                            self.expression(expression.clone(), TranslationHint::ValueExpression)?;
                        let element_type = Type::from_element(&element, self.scope())?;
                        result
                            .push(element_type)
                            .map_err(|error| Error::LiteralArray(location, error))?;
                    }
                    break;
                }
                None => {
                    let element = self.expression(expression, TranslationHint::ValueExpression)?;
                    let element_type = Type::from_element(&element, self.scope())?;
                    result
                        .push(element_type)
                        .map_err(|error| Error::LiteralArray(location, error))?
                }
            }
        }

        Ok(Element::Value(Value::Array(result)))
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = Tuple::default();
        for expression in tuple.elements.into_iter() {
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result.push(element_type);
        }

        Ok(Element::Value(Value::Tuple(result)))
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<Element, Error> {
        let path_location = structure.path.location;
        let (structure_identifier, structure_fields) =
            match self.expression(structure.path, TranslationHint::TypeExpression)? {
                Element::Type(Type::Structure {
                    identifier, fields, ..
                }) => (identifier, fields),
                element => {
                    return Err(Error::TypeAliasDoesNotPointToStructure(
                        path_location,
                        element.to_string(),
                    ))
                }
            };

        let mut result = Structure::new(
            structure_identifier.clone(),
            Vec::with_capacity(structure.fields.len()),
        );
        for (identifier, expression) in structure.fields.into_iter() {
            let location = identifier.location;
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            let field = structure_fields
                .iter()
                .find(|(field_name, _field_type)| field_name == &identifier.name);

            match field {
                Some((_field_name, field_type)) => {
                    if field_type != &element_type {
                        return Err(Error::LiteralStructure(
                            location,
                            StructureValueError::FieldInvalidType(
                                identifier.name,
                                field_type.to_string(),
                                element_type.to_string(),
                            ),
                        ));
                    }
                }
                None => {
                    return Err(Error::LiteralStructure(
                        location,
                        StructureValueError::FieldDoesNotExist(
                            identifier.name,
                            structure_identifier,
                        ),
                    ))
                }
            }

            result
                .push(identifier.name, element_type)
                .map_err(|error| Error::LiteralStructure(location, error))?;
        }

        Ok(Element::Value(Value::Structure(result)))
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<Element, Error> {
        let mut elements = Vec::with_capacity(list.len());
        for expression in list.into_iter() {
            let element = self.expression(expression, TranslationHint::ValueExpression)?;
            elements.push(element);
        }
        Ok(Element::ArgumentList(elements))
    }

    fn translate_path(
        &mut self,
        path: &Path,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        let location = path.location;
        let path_last = path.last();

        match translation_hint {
            TranslationHint::ValueExpression => match Scope::resolve_path(self.scope(), path)? {
                ScopeItem::Variable(variable) => {
                    let size = variable.r#type.size();
                    let value = Value::new(variable.r#type);
                    self.bytecode.borrow_mut().push_instruction_load(
                        variable.address,
                        size,
                        None,
                        false,
                        location,
                    );
                    Ok(Element::Value(value))
                }
                ScopeItem::Constant(constant) => {
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(constant.to_instruction(), location);
                    Ok(Element::Constant(constant))
                }
                ScopeItem::Static(r#static) => {
                    let r#type = r#static.data.r#type();
                    let size = r#type.size();
                    self.bytecode.borrow_mut().push_instruction_load(
                        r#static.address,
                        size,
                        None,
                        true,
                        location,
                    );
                    Ok(Element::Constant(r#static.data))
                }
                ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
            },
            TranslationHint::TypeExpression => match Scope::resolve_path(self.scope(), path)? {
                ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                _ => Ok(Element::Path(path.to_owned())),
            },

            TranslationHint::PathExpression => Ok(Element::Path(path.to_owned())),
            TranslationHint::PlaceExpression => match Scope::resolve_path(self.scope(), path)? {
                ScopeItem::Variable(variable) => Ok(Element::Place(Place::new(
                    location,
                    variable.r#type,
                    variable.address,
                    variable.is_mutable,
                    false,
                ))),
                ScopeItem::Static(r#static) => Ok(Element::Place(Place::new(
                    location,
                    r#static.data.r#type(),
                    r#static.address,
                    false,
                    true,
                ))),
                ScopeItem::Constant(constant) => Ok(Element::Constant(constant)),
                ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                ScopeItem::Module(_) => Ok(Element::Module(path_last.name.to_owned())),
            },
            TranslationHint::CompoundTypeMember => Ok(Element::MemberString(MemberString::new(
                location,
                path_last.name.to_owned(),
            ))),
        }
    }

    fn translate_place(
        &mut self,
        place: &Place,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        match translation_hint {
            TranslationHint::ValueExpression => {
                self.bytecode.borrow_mut().push_instruction_load(
                    place.address,
                    place.r#type.size(),
                    Some(place.total_size),
                    place.is_global,
                    place.location,
                );
                Ok(Element::Value(Value::new(place.r#type.to_owned())))
            }
            _ => Ok(Element::Place(place.to_owned())),
        }
    }

    fn evaluate_operand(&mut self, translation_hint: TranslationHint) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Constant(Constant::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => self.boolean_literal(literal),
                ExpressionOperand::LiteralInteger(literal) => self.integer_literal(literal),
                ExpressionOperand::LiteralString(literal) => self.string_literal(literal),
                ExpressionOperand::MemberInteger(integer) => self.member_integer(integer),
                ExpressionOperand::MemberString(identifier) => self.member_string(identifier),
                ExpressionOperand::Identifier(identifier) => {
                    self.identifier(identifier, translation_hint)
                }
                ExpressionOperand::ExpressionList(expressions) => self.list_expression(expressions),
                ExpressionOperand::Type(r#type) => self.r#type(r#type),
                ExpressionOperand::Block(expression) => {
                    self.push_scope();
                    let result = self.block_expression(expression)?;
                    self.pop_scope();
                    Ok(result)
                }
                ExpressionOperand::Conditional(expression) => {
                    self.conditional_expression(expression)
                }
                ExpressionOperand::Match(expression) => self.match_expression(expression),
                ExpressionOperand::Array(expression) => self.array_expression(expression),
                ExpressionOperand::Tuple(expression) => self.tuple_expression(expression),
                ExpressionOperand::Structure(expression) => self.structure_expression(expression),
            },
            StackElement::Evaluated(element) => match element {
                Element::Path(path) => self.translate_path(&path, translation_hint),
                Element::Place(place) => self.translate_place(&place, translation_hint),
                element => Ok(element),
            },
        }
    }

    fn evaluate_unary_operand(
        &mut self,
        translation_hint: TranslationHint,
    ) -> Result<Element, Error> {
        self.evaluate_operand(translation_hint)
    }

    fn evaluate_binary_operands(
        &mut self,
        translation_hint_1: TranslationHint,
        translation_hint_2: TranslationHint,
    ) -> Result<(Element, Element), Error> {
        self.swap_top();
        let operand_1 = self.evaluate_operand(translation_hint_1)?;
        let operand_2 = self.evaluate_operand(translation_hint_2)?;
        Ok((operand_1, operand_2))
    }

    fn push_operand(&mut self, operand: StackElement) {
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_AN_OPERAND)
    }

    fn swap_top(&mut self) {
        let last_index = self.operands.len() - 1;
        let last_but_one_index = self.operands.len() - 2;
        self.operands.swap(last_index, last_but_one_index)
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(Scope::new_child(self.scope()));
    }

    fn pop_scope(&mut self) {
        self.scope_stack
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }
}
