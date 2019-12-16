//!
//! The expression semantic analyzer.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::semantic::Array;
use crate::semantic::Bytecode;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::IntegerConstant;
use crate::semantic::Place;
use crate::semantic::ResolutionHint;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::ScopeVariableItem;
use crate::semantic::StatementAnalyzer;
use crate::semantic::Structure;
use crate::semantic::Tuple;
use crate::semantic::Type;
use crate::semantic::Value;
use crate::syntax;
use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpression;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;
use crate::syntax::PatternVariant;
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
        resolution_mode: ResolutionHint,
    ) -> Result<Element, Error> {
        let location = expression.location;
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::PlaceExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    let place = operand_1
                        .assign(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = Type::from_element(&operand_2, self.scope())?;

                    let item = Scope::resolve_place(self.scope(), &place)?;
                    let variable = match item {
                        ScopeItem::Variable(variable) => variable,
                        item => {
                            return Err(Error::AssignmentToInvalidItem(location, item.to_string()))
                        }
                    };
                    if !variable.is_mutable {
                        return Err(Error::AssignmentToImmutableMemory(location, place));
                    }
                    let size = variable.r#type.size();
                    if variable.r#type != r#type {
                        return Err(Error::AssignmentTypesMismatch(
                            location,
                            r#type.to_string(),
                            place,
                            variable.r#type.to_string(),
                        ));
                    }

                    self.bytecode
                        .borrow_mut()
                        .push_instruction_pop_store(variable.address, size);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE);
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE);
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Or(zinc_bytecode::Or));

                    let result = operand_1
                        .or(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Xor(zinc_bytecode::Xor));

                    let result = operand_1
                        .xor(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::And(zinc_bytecode::And));

                    let result = operand_1
                        .and(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq));

                    let result = operand_1
                        .equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ne(zinc_bytecode::Ne));

                    let result = operand_1
                        .not_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Ge(zinc_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Le(zinc_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Gt(zinc_bytecode::Gt));

                    let result = operand_1
                        .greater(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Lt(zinc_bytecode::Lt));

                    let result = operand_1
                        .lesser(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Add(zinc_bytecode::Add));

                    let result = operand_1
                        .add(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Sub(zinc_bytecode::Sub));

                    let result = operand_1
                        .subtract(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Mul(zinc_bytecode::Mul));

                    let result = operand_1
                        .multiply(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Div(zinc_bytecode::Div));

                    let result = operand_1
                        .divide(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::ValueExpression,
                    )?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Rem(zinc_bytecode::Rem));

                    let result = operand_1
                        .remainder(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::ValueExpression,
                        ResolutionHint::TypeExpression,
                    )?;

                    if let Some((is_signed, bitlength)) = operand_1
                        .cast(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?
                    {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Cast(zinc_bytecode::Cast::new(
                                is_signed,
                                bitlength as u8,
                            )));
                    }

                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.evaluate_unary_operand(ResolutionHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Neg(zinc_bytecode::Neg));

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.evaluate_unary_operand(ResolutionHint::ValueExpression)?;
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Not(zinc_bytecode::Not));

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::PlaceExpression,
                        ResolutionHint::ValueExpression,
                    )?;

                    operand_1
                        .index(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::PlaceExpression,
                        ResolutionHint::CompoundTypeMember,
                    )?;

                    operand_1
                        .field(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::InstructionCall) => {
                    self.is_next_call_instruction = true;
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    // check if the call is a direct instruction call like 'dbg' or 'assert'
                    let is_instruction = self.is_next_call_instruction;
                    self.is_next_call_instruction = false;

                    if !is_instruction {
                        self.bytecode.borrow_mut().push_call_stack_pointer();
                    }

                    let (operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::TypeExpression,
                        ResolutionHint::ValueExpression,
                    )?;

                    // check if the first operand is a function and get its data
                    let (identifier, argument_types, return_type) = match operand_1 {
                        Element::Type(Type::Function {
                            identifier,
                            arguments,
                            return_type,
                        }) => (identifier, arguments, return_type),
                        Element::Place(place) => {
                            match Scope::resolve_place(self.scope(), &place)? {
                                ScopeItem::Type(Type::Function {
                                    identifier,
                                    arguments,
                                    return_type,
                                }) => (identifier, arguments, return_type),
                                item => {
                                    return Err(Error::FunctionCallingNotCallableObject(
                                        element.location,
                                        item.to_string(),
                                    ))
                                }
                            }
                        }
                        operand => {
                            return Err(Error::FunctionCallingNotCallableObject(
                                element.location,
                                operand.to_string(),
                            ))
                        }
                    };

                    // check the number of the arguments
                    let argument_values = match operand_2 {
                        Element::ArgumentList(values) => values,
                        _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                    let argument_types_count = argument_types.len();
                    let argument_values_count = argument_values.len();

                    if !is_instruction {
                        if argument_values.len() != argument_types.len() {
                            return Err(Error::FunctionArgumentCountMismatch(
                                element.location,
                                identifier,
                                argument_types_count,
                                argument_values_count,
                            ));
                        }

                        // check the argument types
                        for (argument_index, (argument_name, expected_type)) in
                            argument_types.into_iter().enumerate()
                        {
                            let actual_type =
                                Type::from_element(&argument_values[argument_index], self.scope())?;
                            if expected_type != actual_type {
                                return Err(Error::FunctionArgumentTypeMismatch(
                                    element.location,
                                    identifier,
                                    argument_name,
                                    expected_type.to_string(),
                                    actual_type.to_string(),
                                ));
                            }
                        }

                        for argument in argument_values.into_iter() {
                            let argument_type = Type::from_element(&argument, self.scope())?;
                            let argument_size = argument_type.size();
                            let argument_address = self
                                .bytecode
                                .borrow_mut()
                                .allocate_stack_space(argument_size);
                            self.bytecode
                                .borrow_mut()
                                .push_instruction_pop_store(argument_address, argument_size);
                        }

                        let function_address = self
                            .bytecode
                            .borrow_mut()
                            .function_address(identifier.as_str())
                            .expect(crate::semantic::PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS);

                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Call(zinc_bytecode::Call::new(
                                function_address,
                                argument_values_count,
                            )));
                        self.bytecode.borrow_mut().pop_call_stack_pointer();
                    } else {
                        let instruction = match identifier.as_str() {
                            "dbg" => {
                                let string = match &argument_values[0] {
                                    Element::Constant(Constant::String(string)) => {
                                        string.to_owned()
                                    }
                                    argument => {
                                        return Err(Error::InstructionDebugExpectedString(
                                            element.location,
                                            argument.to_string(),
                                        ))
                                    }
                                };
                                Instruction::Log(zinc_bytecode::Dbg::new(
                                    string,
                                    argument_values_count,
                                ))
                            }
                            "assert" => Instruction::Assert(zinc_bytecode::Assert),
                            identifier => {
                                return Err(Error::FunctionNotInstruction(
                                    element.location,
                                    identifier.to_owned(),
                                ))
                            }
                        };
                        self.bytecode.borrow_mut().push_instruction(instruction);
                    }

                    self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                        *return_type,
                    ))));
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (mut operand_1, operand_2) = self.evaluate_binary_operands(
                        ResolutionHint::PlaceExpression,
                        ResolutionHint::CompoundTypeMember,
                    )?;

                    operand_1
                        .path(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
            }
        }

        self.evaluate_operand(resolution_mode)
    }

    pub fn block_expression(&mut self, block: BlockExpression) -> Result<Element, Error> {
        for statement in block.statements.into_iter() {
            StatementAnalyzer::new(self.scope(), self.bytecode.clone(), HashMap::new())
                .inner_statement(statement)?;
        }
        match block.expression {
            Some(expression) => self.expression(*expression, ResolutionHint::ValueExpression),
            None => Ok(Element::Value(Value::Unit)),
        }
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let constant = Constant::from(literal);
        self.bytecode
            .borrow_mut()
            .push_instruction(constant.clone().into());
        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let integer = IntegerConstant::try_from(&literal)
            .map_err(|error| Error::InferenceConstant(location, error))?;
        self.bytecode
            .borrow_mut()
            .push_instruction(integer.clone().into());
        Ok(Element::Constant(Constant::Integer(integer)))
    }

    fn string_literal(&mut self, literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }

    fn member_integer(&mut self, integer: MemberInteger) -> Result<Element, Error> {
        let integer = integer.literal.into();
        Ok(Element::MemberInteger(integer))
    }

    fn member_string(&mut self, member_string: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_string))
    }

    fn identifier(
        &mut self,
        identifier: Identifier,
        resolution_mode: ResolutionHint,
    ) -> Result<Element, Error> {
        let location = identifier.location;

        match resolution_mode {
            ResolutionHint::ValueExpression => {
                match Scope::resolve_item(self.scope(), &identifier.name)
                    .map_err(|error| Error::Scope(location, error))?
                {
                    ScopeItem::Variable(variable) => {
                        let size = variable.r#type.size();
                        let value = Value::new(variable.r#type);
                        self.bytecode
                            .borrow_mut()
                            .push_instruction_load_push(variable.address, size);
                        Ok(Element::Value(value))
                    }
                    ScopeItem::Constant(constant) => {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(constant.clone().into());
                        Ok(Element::Constant(constant))
                    }
                    ScopeItem::Static(r#static) => {
                        let r#type = r#static.data.r#type();
                        let size = r#type.size();
                        self.bytecode
                            .borrow_mut()
                            .push_instruction_load_push(r#static.address, size);
                        Ok(Element::Constant(r#static.data))
                    }
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(identifier.name)),
                }
            }
            ResolutionHint::PlaceExpression => Ok(Element::Place(Place::new(
                location,
                MemberString::from(identifier),
            ))),
            ResolutionHint::TypeExpression => {
                match Scope::resolve_item(self.scope(), &identifier.name)
                    .map_err(|error| Error::Scope(location, error))?
                {
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    _ => Ok(Element::Place(Place::new(
                        location,
                        MemberString::from(identifier),
                    ))),
                }
            }
            ResolutionHint::CompoundTypeMember => {
                Ok(Element::MemberString(MemberString::from(identifier)))
            }
        }
    }

    fn r#type(&mut self, r#type: syntax::Type) -> Result<Element, Error> {
        Type::from_type_variant(r#type.variant, self.scope()).map(Element::Type)
    }

    fn conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        // compile the condition and check if it is boolean
        let condition_result =
            self.expression(*conditional.condition, ResolutionHint::ValueExpression)?;
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
            .push_instruction(Instruction::If(zinc_bytecode::If));

        self.push_scope();
        let main_result = self.block_expression(conditional.main_block)?;
        let main_type = Type::from_element(&main_result, self.scope())?;
        self.pop_scope();

        let else_type = if let Some(else_block) = conditional.else_block {
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_bytecode::Else));

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
            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));

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

        let scrutinee_result =
            self.expression(r#match.scrutinee, ResolutionHint::ValueExpression)?;
        let scrutinee_type = Type::from_element(&scrutinee_result, self.scope())?;
        let scrutinee_size = scrutinee_type.size();
        let scrutinee_address = self
            .bytecode
            .borrow_mut()
            .allocate_stack_space(scrutinee_size);
        self.bytecode
            .borrow_mut()
            .push_instruction_pop_store(scrutinee_address, scrutinee_size);

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
                PatternVariant::BooleanLiteral(boolean) => {
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
                            .push_instruction(Instruction::Else(zinc_bytecode::Else));
                        endifs += 1;
                    }

                    self.bytecode
                        .borrow_mut()
                        .push_instruction_load_push(scrutinee_address, scrutinee_size);
                    self.bytecode.borrow_mut().push_instruction(constant.into());
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq));
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If));

                    self.expression(expression, ResolutionHint::ValueExpression)?
                }
                PatternVariant::IntegerLiteral(integer) => {
                    let constant = IntegerConstant::try_from(&integer)
                        .map_err(|error| Error::InferencePatternMatch(integer.location, error))?;
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
                            .push_instruction(Instruction::Else(zinc_bytecode::Else));
                        endifs += 1;
                    }

                    self.bytecode
                        .borrow_mut()
                        .push_instruction_load_push(scrutinee_address, scrutinee_size);
                    self.bytecode.borrow_mut().push_instruction(constant.into());
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Eq(zinc_bytecode::Eq));
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If));

                    self.expression(expression, ResolutionHint::ValueExpression)?
                }
                PatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else));
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
                    let result = self.expression(expression, ResolutionHint::ValueExpression)?;
                    self.pop_scope();

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));
                    }

                    result
                }
                PatternVariant::Ignoring => {
                    is_exhausted = true;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Else(zinc_bytecode::Else));
                    }

                    let result = self.expression(expression, ResolutionHint::ValueExpression)?;

                    if index > 0 {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));
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
                .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));
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
            let element = self.expression(expression, ResolutionHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result
                .push(element_type)
                .map_err(|error| Error::LiteralArray(location, error))?;
        }

        Ok(Element::Value(Value::Array(result)))
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = Tuple::default();
        for expression in tuple.elements.into_iter() {
            let element = self.expression(expression, ResolutionHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result.push(element_type);
        }

        Ok(Element::Value(Value::Tuple(result)))
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<Element, Error> {
        let path_location = structure.path.location;
        let identifier = match self.expression(structure.path, ResolutionHint::TypeExpression)? {
            Element::Type(Type::Structure { identifier, .. }) => identifier,
            element => {
                return Err(Error::TypeAliasDoesNotPointToStructure(
                    path_location,
                    element.to_string(),
                ))
            }
        };

        let mut result = Structure::new(identifier, Vec::with_capacity(structure.fields.len()));
        for (identifier, expression) in structure.fields.into_iter() {
            let location = identifier.location;
            let element = self.expression(expression, ResolutionHint::ValueExpression)?;
            let element_type = Type::from_element(&element, self.scope())?;
            result
                .push(identifier.name, element_type)
                .map_err(|error| Error::LiteralStructure(location, error))?;
        }

        Ok(Element::Value(Value::Structure(result)))
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<Element, Error> {
        let mut elements = Vec::with_capacity(list.len());
        for expression in list.into_iter().rev() {
            let element = self.expression(expression, ResolutionHint::ValueExpression)?;
            elements.push(element);
        }
        Ok(Element::ArgumentList(elements))
    }

    fn evaluate_operand(&mut self, resolution_mode: ResolutionHint) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Constant(Constant::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => self.boolean_literal(literal),
                ExpressionOperand::LiteralInteger(literal) => self.integer_literal(literal),
                ExpressionOperand::LiteralString(literal) => self.string_literal(literal),
                ExpressionOperand::MemberInteger(integer) => self.member_integer(integer),
                ExpressionOperand::MemberString(identifier) => self.member_string(identifier),
                ExpressionOperand::Identifier(identifier) => {
                    self.identifier(identifier, resolution_mode)
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
            StackElement::Evaluated(element) => match (resolution_mode, element) {
                (ResolutionHint::ValueExpression, Element::Place(place)) => {
                    match Scope::resolve_place(self.scope(), &place)? {
                        ScopeItem::Variable(variable) => {
                            let size = variable.r#type.size();
                            self.bytecode
                                .borrow_mut()
                                .push_instruction_load_push(variable.address, size);
                            let value = Value::new(variable.r#type);
                            Ok(Element::Value(value))
                        }
                        ScopeItem::Constant(constant) => {
                            self.bytecode
                                .borrow_mut()
                                .push_instruction(constant.clone().into());
                            Ok(Element::Constant(constant))
                        }
                        ScopeItem::Static(r#static) => {
                            let size = r#static.data.r#type().size();
                            self.bytecode
                                .borrow_mut()
                                .push_instruction_load_push(r#static.address, size);
                            Ok(Element::Constant(r#static.data))
                        }
                        _ => Ok(Element::Place(place)),
                    }
                }
                (_resolution_mode, element) => Ok(element),
            },
        }
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scope_stack
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    fn push_scope(&mut self) {
        self.scope_stack
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) {
        self.scope_stack
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }

    fn evaluate_unary_operand(
        &mut self,
        resolution_mode: ResolutionHint,
    ) -> Result<Element, Error> {
        self.evaluate_operand(resolution_mode)
    }

    fn evaluate_binary_operands(
        &mut self,
        resolution_mode_1: ResolutionHint,
        resolution_mode_2: ResolutionHint,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand(resolution_mode_2)?;
        let operand_1 = self.evaluate_operand(resolution_mode_1)?;
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
}
