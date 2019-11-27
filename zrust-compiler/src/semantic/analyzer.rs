//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

use zrust_bytecode::dispatch_instruction;
use zrust_bytecode::Instruction;
use zrust_bytecode::InstructionInfo;

use crate::error::Error as CompilerError;
use crate::lexical::Literal as InnerLiteral;
use crate::semantic::inference;
use crate::semantic::Element;
use crate::semantic::ElementError;
use crate::semantic::Error;
use crate::semantic::Integer;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::Value;
use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::InnerStatement;
use crate::syntax::Literal;
use crate::syntax::OuterStatement;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;
use crate::syntax::Type;
use crate::syntax::TypeVariant;
use crate::CircuitProgram;

pub struct Analyzer {
    scopes: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    instructions: Vec<Instruction>,
    stack_height: usize,
    function_addresses: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
enum StackElement {
    NotEvaluated(ExpressionOperand),
    Evaluated(Element),
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;
    const STACK_OPERAND_INITIAL_CAPACITY: usize = 16;
    const HASHMAP_FUNCTIONS_INITIAL_CAPACITY: usize = 16;
    const VECTOR_INSTRUCTION_INITIAL_CAPACITY: usize = 1024;

    pub fn new(scope: Scope) -> Self {
        Self {
            scopes: {
                let mut scopes = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scopes.push(Rc::new(RefCell::new(scope)));
                scopes
            },
            operands: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
            instructions: Vec::with_capacity(Self::VECTOR_INSTRUCTION_INITIAL_CAPACITY),
            stack_height: 0,
            function_addresses: HashMap::with_capacity(Self::HASHMAP_FUNCTIONS_INITIAL_CAPACITY),
        }
    }

    pub fn compile(mut self, program: CircuitProgram) -> Result<Vec<Instruction>, CompilerError> {
        // insert the placeholders the 'main' function call
        self.push_instruction(Instruction::NoOperation(zrust_bytecode::NoOperation));
        self.push_instruction(Instruction::NoOperation(zrust_bytecode::NoOperation));

        // compile all the outer statements which generally only declare new items
        for statement in program.statements.into_iter() {
            self.outer_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        // replace the placeholders inserted above with an actual 'main' function call
        let main_function_address = self
            .function_addresses
            .get("main")
            .copied()
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;
        match self
            .scope()
            .borrow()
            .resolve_type("main")
            .expect(crate::semantic::PANIC_RESOLUTION_FUNCTION_MAIN)
        {
            TypeVariant::Function {
                arguments,
                return_type,
                ..
            } => {
                self.instructions[0] = Instruction::Call(zrust_bytecode::Call::new(
                    main_function_address,
                    arguments
                        .into_iter()
                        .map(|(_arg_name, arg_type)| arg_type.size())
                        .sum(),
                ));
                self.instructions[1] =
                    Instruction::Exit(zrust_bytecode::Exit::new(return_type.size()));
            }
            _ => panic!(crate::semantic::PANIC_RESOLUTION_FUNCTION_MAIN),
        }

        Ok(self.instructions)
    }

    fn outer_statement(&mut self, statement: OuterStatement) -> Result<(), Error> {
        match statement {
            OuterStatement::Type(r#type) => {
                let location = r#type.location;
                let type_variant = TypeVariant::new_alias(r#type.identifier.name.clone());
                self.scope()
                    .borrow_mut()
                    .declare_type(r#type.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Struct(r#struct) => {
                let location = r#struct.location;
                let type_variant = TypeVariant::new_structure(
                    r#struct.identifier.clone(),
                    r#struct
                        .fields
                        .into_iter()
                        .map(|field| (field.identifier.name, field.r#type.variant))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#struct.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Enum(r#enum) => {
                let location = r#enum.location;
                let type_variant = TypeVariant::new_enumeration(
                    r#enum.identifier.clone(),
                    r#enum
                        .variants
                        .into_iter()
                        .map(|variant| (variant.identifier.name, variant.literal))
                        .collect(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#enum.identifier.name, type_variant)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Fn(r#fn) => {
                let location = r#fn.location;

                // declare the function as a new type
                let type_variant = TypeVariant::new_function(
                    r#fn.identifier.clone(),
                    r#fn.arguments
                        .iter()
                        .map(|field| (field.identifier.name.clone(), field.r#type.variant.clone()))
                        .collect(),
                    r#fn.return_type.variant.clone(),
                );
                self.scope()
                    .borrow_mut()
                    .declare_type(r#fn.identifier.name.clone(), type_variant)
                    .map_err(|error| Error::Scope(location, error))?;

                // record the function address in the bytecode
                self.function_addresses
                    .insert(r#fn.identifier.name.clone(), self.instructions.len());

                // reset the stack frame address counter
                self.stack_height = 0;

                // start a new scope and declare the function arguments there
                self.push_scope();
                for argument in r#fn.arguments.into_iter() {
                    self.stack_height += 1;
                    self.scope()
                        .borrow_mut()
                        .declare_variable(
                            argument.identifier.name,
                            argument.r#type.variant,
                            false,
                            self.stack_last_index(),
                        )
                        .map_err(|error| Error::Scope(location, error))?;
                }

                // compile the function block
                self.block_expression(r#fn.body)?;

                // get the function block result and pop the scope
                let result = self.evaluate_unary_operand(true)?;
                let return_type = result
                    .type_variant(self.scope().borrow().deref())
                    .map_err(|error| Error::Element(location, error))?;
                self.push_operand(StackElement::Evaluated(result));
                self.pop_scope();

                // check the function return type to match the block result
                if r#fn.return_type.variant != return_type {
                    return Err(Error::FunctionReturnTypeMismatch(
                        r#fn.return_type.location,
                        r#fn.identifier.name,
                        r#fn.return_type.variant,
                        return_type,
                    ));
                }

                self.push_instruction(Instruction::Return(zrust_bytecode::Return::new(
                    r#fn.return_type.variant.size(),
                )));
            }
            OuterStatement::Mod(_mod) => {}
            OuterStatement::Use(_use) => {}
        }

        Ok(())
    }

    fn inner_statement(&mut self, statement: InnerStatement) -> Result<(), Error> {
        match statement {
            InnerStatement::Empty => {}
            InnerStatement::Let(r#let) => {
                let location = r#let.location;

                // compile the expression being assigned
                self.expression(r#let.expression)?;

                let type_variant = if let Some(r#type) = r#let.r#type {
                    // get and resolve the type
                    let let_type_variant = match r#type.variant {
                        TypeVariant::Alias { identifier } => {
                            let location = r#type.location;
                            self.scope()
                                .borrow()
                                .resolve_type(&identifier)
                                .map_err(|error| Error::Scope(location, error))?
                        }
                        type_variant => type_variant,
                    };

                    // get the expression result try to cast the expression to the specified type
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    let (is_signed, bitlength) = operand_1
                        .cast(
                            Element::Type(let_type_variant.clone()),
                            self.scope().borrow().deref(),
                        )
                        .map_err(|error| Error::Element(location, error))?;
                    self.push_instruction(Instruction::Cast(zrust_bytecode::Cast::new(
                        is_signed,
                        bitlength as u8,
                    )));
                    let_type_variant
                } else {
                    // just get the expression result
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    operand_1
                        .type_variant(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(location, error))?
                };

                // declare the variable
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        r#let.identifier.name,
                        type_variant,
                        r#let.is_mutable,
                        self.stack_last_index(),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
            InnerStatement::Loop(r#loop) => {
                let location = r#loop.location;

                // infer the bitlength of the range start and end
                let range_bitlength =
                    inference::enough_bitlength(&[&r#loop.range_start, &r#loop.range_end])
                        .map_err(|error| Error::LoopBoundsTypeInference(location, error))?;

                // calculate the iterations number and if the loop is reverse
                let range_start: usize = r#loop.range_start.into();
                let range_end: usize = r#loop.range_end.into();
                let iterations_count = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if r#loop.is_range_inclusive { 1 } else { 0 };
                let is_reverse = range_start > range_end;

                // create the index value and get its address
                let index = Integer::new_range_bound(range_start, range_bitlength);
                self.push_instruction(Instruction::Push(index.to_push()));
                let index_address = self.stack_last_index();

                // declare the index variable
                self.push_scope();
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        r#loop.index_identifier.name,
                        TypeVariant::new_integer_unsigned(range_bitlength),
                        false,
                        index_address,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                self.push_instruction(Instruction::NoOperation(zrust_bytecode::NoOperation));
                let loop_begin_index = self.instructions.len() - 1;
                self.block_expression(r#loop.block)?;
                let scope = self.pop_scope();
                let outputs = scope.get_ordered_outer_assignments();
                let outputs_count = outputs.len() + 1;
                self.instructions[loop_begin_index] = Instruction::LoopBegin(
                    zrust_bytecode::LoopBegin::new(iterations_count, outputs_count),
                );
                for (_place, assignment) in outputs.iter() {
                    self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                        assignment.address,
                    )));
                }
                self.push_instruction(Instruction::Push(
                    Integer::new_one(range_bitlength).to_push(),
                ));
                self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(index_address)));
                self.push_instruction(if is_reverse {
                    Instruction::Sub(zrust_bytecode::Sub)
                } else {
                    Instruction::Add(zrust_bytecode::Add)
                });
                self.push_instruction(Instruction::LoopEnd(zrust_bytecode::LoopEnd));
                self.stack_height -= outputs_count;
            }
            InnerStatement::Expression(expression) => {
                // just calculate the expression (TODO: remove its result from the bytecode)
                self.expression(expression)?;
            }
        }

        Ok(())
    }

    fn expression(&mut self, expression: Expression) -> Result<(), Error> {
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(false, true)?;
                    let place = operand_1
                        .assign(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.scope()
                        .borrow_mut()
                        .update_variable(place, self.stack_last_index())
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => unimplemented!(),
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Or(zrust_bytecode::Or));

                    let result = operand_1
                        .or(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Xor(zrust_bytecode::Xor));

                    let result = operand_1
                        .xor(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::And(zrust_bytecode::And));

                    let result = operand_1
                        .and(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Eq(zrust_bytecode::Eq));

                    let result = operand_1
                        .equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Ne(zrust_bytecode::Ne));

                    let result = operand_1
                        .not_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Ge(zrust_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Le(zrust_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Gt(zrust_bytecode::Gt));

                    let result = operand_1
                        .greater(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Lt(zrust_bytecode::Lt));

                    let result = operand_1
                        .lesser(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Add(zrust_bytecode::Add));

                    let result = operand_1
                        .add(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Sub(zrust_bytecode::Sub));

                    let result = operand_1
                        .subtract(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Mul(zrust_bytecode::Mul));

                    let result = operand_1
                        .multiply(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Div(zrust_bytecode::Div));

                    let result = operand_1
                        .divide(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.push_instruction(Instruction::Rem(zrust_bytecode::Rem));

                    let result = operand_1
                        .remainder(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, false)?;
                    let (is_signed, bitlength) = operand_1
                        .cast(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_instruction(Instruction::Cast(zrust_bytecode::Cast::new(
                        is_signed,
                        bitlength as u8,
                    )));
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                        TypeVariant::new_integer(is_signed, bitlength),
                    ))));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    self.push_instruction(Instruction::Neg(zrust_bytecode::Neg));

                    let result = operand_1
                        .negate(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    self.push_instruction(Instruction::Not(zrust_bytecode::Not));

                    let result = operand_1
                        .not(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(false, false)?;

                    let result = operand_1
                        .index(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(false, false)?;

                    let result = operand_1
                        .field(operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(false, false)?;

                    // check if the first operand is a function and get its data
                    let (identifier, argument_types, return_type) = match operand_1 {
                        Element::Type(TypeVariant::Function {
                            identifier,
                            arguments,
                            return_type,
                        }) => (identifier, arguments, return_type),
                        operand => {
                            return Err(Error::FunctionCallOnNotCallable(element.location, operand))
                        }
                    };

                    // check the number of the arguments
                    let argument_values = match operand_2 {
                        Element::ValueList(values) => values,
                        _ => panic!("Ensured to be an expression list during the syntax analysis"),
                    };
                    let argument_types_count = argument_types.len();
                    let argument_values_count = argument_values.len();
                    if argument_values.len() != argument_types.len() {
                        return Err(Error::FunctionArgumentCountMismatch(
                            element.location,
                            identifier.name,
                            argument_types_count,
                            argument_values_count,
                        ));
                    }

                    // check the argument types
                    for (argument_index, (argument_name, expected_type)) in
                        argument_types.into_iter().enumerate()
                    {
                        let actual_type = argument_values[argument_index].type_variant();
                        if expected_type != actual_type {
                            return Err(Error::FunctionArgumentTypeMismatch(
                                element.location,
                                identifier.name,
                                argument_name,
                                expected_type,
                                actual_type,
                            ));
                        }
                    }

                    // get the function address
                    let function_address = self
                        .function_addresses
                        .get(&identifier.name)
                        .copied()
                        .expect("Presence checked during the function type resolution");

                    self.push_instruction(Instruction::Call(zrust_bytecode::Call::new(
                        function_address,
                        argument_values_count,
                    )));

                    self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                        *return_type,
                    ))))
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => unimplemented!(),
            }
        }

        let element = self.evaluate_operand(true)?;
        self.push_operand(StackElement::Evaluated(element));

        Ok(())
    }

    fn literal(&mut self, literal: Literal) -> Result<(), Error> {
        let location = literal.location;

        let value = match literal.data {
            InnerLiteral::Boolean(literal) => Value::from(literal),
            InnerLiteral::Integer(literal) => Value::try_from(literal)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(location, error))?,
            InnerLiteral::String { .. } => panic!("String literals cannot be used in expressions"),
        };
        self.push_instruction(Instruction::Push(value.to_push()));
        self.push_operand(StackElement::Evaluated(Element::Value(value)));

        Ok(())
    }

    fn identifier(&mut self, identifier: Identifier, is_for_stack: bool) -> Result<(), Error> {
        let location = identifier.location;

        if is_for_stack {
            // put the identifier as an 'rvalue'
            let place = Place::new(identifier.name.clone());
            let address = self
                .scope()
                .borrow()
                .get_variable_address(&place)
                .map_err(|error| Error::Scope(location, error))?;
            let value = self
                .scope()
                .borrow()
                .get_declaration(&place.identifier)
                .map(|declaration| Value::new(declaration.type_variant))
                .map_err(|error| Error::Scope(location, error))?;
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(address)));
            self.push_operand(StackElement::Evaluated(Element::Value(value)));
        } else {
            // put the identifier as an 'lvalue' part
            let item_type = self
                .scope()
                .borrow()
                .get_item_type(&identifier.name)
                .map_err(|error| Error::Scope(location, error))?;
            match item_type {
                ScopeItem::Variable => {
                    let place = Place::new(identifier.name.clone());
                    self.push_operand(StackElement::Evaluated(Element::Place(place)))
                }
                ScopeItem::Type => {
                    let type_variant = self
                        .scope()
                        .borrow()
                        .resolve_type(&identifier.name)
                        .map_err(|error| Error::Scope(location, error))?;
                    self.push_operand(StackElement::Evaluated(Element::Type(type_variant)));
                }
                ScopeItem::Variant => {}
            }
        }

        Ok(())
    }

    fn r#type(&mut self, r#type: Type) -> Result<(), Error> {
        let location = r#type.location;
        let resolved_type_variant = match r#type.variant {
            TypeVariant::Alias { identifier } => self
                .scope()
                .borrow()
                .resolve_type(&identifier)
                .map_err(|error| Error::Scope(location, error))?,
            type_variant => type_variant,
        };
        self.push_operand(StackElement::Evaluated(Element::Type(
            resolved_type_variant,
        )));

        Ok(())
    }

    fn block_expression(&mut self, block: BlockExpression) -> Result<(), Error> {
        for statement in block.statements.into_iter() {
            self.inner_statement(statement)?;
        }
        if let Some(expression) = block.expression {
            self.expression(*expression)?;
        } else {
            self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
        }

        Ok(())
    }

    fn conditional_expression(&mut self, conditional: ConditionalExpression) -> Result<(), Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        // remember the stack address before the conditional expression
        let outer_stack_frame_start = self.stack_height;
        // the 'if' itself is also wrapped into a frame to release its outputs and condition
        self.push_instruction(Instruction::FrameBegin(zrust_bytecode::FrameBegin));

        // compile the condition and check if it is boolean
        self.expression(*conditional.condition)?;
        let condition_address = self.stack_last_index();
        match self
            .evaluate_unary_operand(true)?
            .type_variant(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?
        {
            TypeVariant::Boolean => {}
            type_variant => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    condition_location,
                    type_variant,
                ))
            }
        }

        // copy the 'true' variant of the condition
        self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
            condition_address,
        )));
        self.push_instruction(Instruction::PushCondition(zrust_bytecode::PushCondition));

        // remember the initial stack position, push the scope and start the frame
        let stack_frame_start = self.stack_height;
        self.push_scope();
        self.push_instruction(Instruction::FrameBegin(zrust_bytecode::FrameBegin));

        // evaluate the main block and get the result
        self.block_expression(conditional.main_block)?;
        let (main_result, main_result_address) =
            (self.evaluate_unary_operand(true)?, self.stack_last_index());
        let main_type = main_result
            .type_variant(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?;

        // pop the scope
        let scope = self.pop_scope();

        // get the ordered list of the outer assignments
        let main_assignments = scope.get_ordered_outer_assignments();
        let main_outputs_count = main_assignments.len() + main_type.size();
        let mut conditional_assignments = HashMap::<Place, (usize, usize)>::new();

        // reassign the outer variables and release the frame
        self.stack_height = stack_frame_start;
        for (place, assignment) in main_assignments.iter() {
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                assignment.address,
            )));
            let old_address = self.scope().borrow().get_variable_address(&place).expect(
                "The variable address always exists as it is checked during the block evaluation",
            );
            conditional_assignments
                .insert(place.to_owned(), (self.stack_last_index(), old_address));
        }
        match main_type {
            TypeVariant::Unit => {}
            _ => self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                main_result_address,
            ))),
        }

        // release the frame
        self.push_instruction(Instruction::FrameEnd(zrust_bytecode::FrameEnd::new(
            main_outputs_count,
        )));

        // pop the condition
        self.push_instruction(Instruction::PopCondition(zrust_bytecode::PopCondition));

        let (else_type, else_result_address) = if let Some(else_block) = conditional.else_block {
            // copy the 'false' variant of the condition
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                condition_address,
            )));
            self.push_instruction(Instruction::Not(zrust_bytecode::Not));
            self.push_instruction(Instruction::PushCondition(zrust_bytecode::PushCondition));

            // remember the initial stack position, push the scope and start the frame
            let stack_frame_start = self.stack_height;
            self.push_scope();
            self.push_instruction(Instruction::FrameBegin(zrust_bytecode::FrameBegin));

            // evaluate the main block and get the result
            self.block_expression(else_block)?;
            let (else_result, else_result_address) =
                (self.evaluate_unary_operand(true)?, self.stack_last_index());
            let else_type = else_result
                .type_variant(self.scope().borrow().deref())
                .map_err(|error| Error::Element(location, error))?;

            // pop the scope
            let scope = self.pop_scope();

            // set the ordered list of the outer assignments
            let else_assignments = scope.get_ordered_outer_assignments();
            let else_outputs_count = else_assignments.len() + else_type.size();

            // reassign the outer variables and release the frame
            self.stack_height = stack_frame_start;
            for (place, assignment) in else_assignments.iter() {
                self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                    assignment.address,
                )));
                let old_address = self.scope().borrow()
                    .get_variable_address(&place)
                    .expect("The variable address always exists as it is checked during the block evaluation");
                if let Some(assignment) = conditional_assignments.get_mut(place) {
                    assignment.1 = self.stack_last_index();
                } else {
                    conditional_assignments
                        .insert(place.to_owned(), (old_address, self.stack_last_index()));
                }
            }
            match else_type {
                TypeVariant::Unit => {}
                _ => self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                    else_result_address,
                ))),
            }

            // release the frame
            self.push_instruction(Instruction::FrameEnd(zrust_bytecode::FrameEnd::new(
                else_outputs_count,
            )));

            // pop the condition
            self.push_instruction(Instruction::PopCondition(zrust_bytecode::PopCondition));

            (else_type, else_result_address)
        } else {
            // the else block is absent, returning a unit value
            self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
            (TypeVariant::Unit, self.stack_last_index())
        };

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypeMismatch(
                location, main_type, else_type,
            ));
        }

        // insert conditional selects for all the outputs
        let mut outputs_count = conditional_assignments.len();
        self.stack_height = outer_stack_frame_start;
        for (place, (address_1, address_2)) in conditional_assignments.into_iter() {
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(address_2)));
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(address_1)));
            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                condition_address,
            )));
            self.push_instruction(Instruction::ConditionalSelect(
                zrust_bytecode::ConditionalSelect,
            ));
            self.scope()
                .borrow_mut()
                .update_variable(place, self.stack_last_index())
                .map_err(|error| Error::Scope(location, error))?;
        }

        match main_type {
            TypeVariant::Unit => {}
            _ => {
                self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                    else_result_address,
                )));
                self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                    main_result_address,
                )));
                self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                    condition_address,
                )));
                self.push_instruction(Instruction::ConditionalSelect(
                    zrust_bytecode::ConditionalSelect,
                ));
                self.push_operand(StackElement::Evaluated(main_result));
                outputs_count += 1;
            }
        }

        self.push_instruction(Instruction::FrameEnd(zrust_bytecode::FrameEnd::new(
            outputs_count,
        )));

        Ok(())
    }

    fn array_expression(&mut self, array: ArrayExpression) -> Result<(), Error> {
        for element in array.elements.into_iter() {
            self.expression(element)?;
            self.pop_operand();
        }

        Ok(())
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<(), Error> {
        for element in tuple.elements.into_iter() {
            self.expression(element)?;
            self.pop_operand();
        }

        Ok(())
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<(), Error> {
        for (_identifier, expression) in structure.fields.into_iter() {
            self.expression(expression)?;
            self.pop_operand();
        }

        Ok(())
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<usize, Error> {
        let input_length = list.len();
        let mut values = Vec::with_capacity(input_length);
        for expression in list.into_iter() {
            self.expression(expression)?;
            match self.evaluate_unary_operand(true)? {
                Element::Value(value) => values.push(value),
                _ => panic!("Is always put as a value by the evaluator above"),
            }
        }
        self.push_operand(StackElement::Evaluated(Element::ValueList(values)));

        Ok(input_length)
    }

    fn evaluate_operand(&mut self, is_for_stack: bool) -> Result<Element, Error> {
        Ok(match self.pop_operand() {
            StackElement::NotEvaluated(operand) => {
                match operand {
                    ExpressionOperand::Unit => {}
                    ExpressionOperand::Literal(literal) => self.literal(literal)?,
                    ExpressionOperand::Identifier(identifier) => {
                        self.identifier(identifier, is_for_stack)?
                    }
                    ExpressionOperand::List(list) => {
                        self.list_expression(list)?;
                    }
                    ExpressionOperand::Type(r#type) => self.r#type(r#type)?,
                    ExpressionOperand::Block(block) => {
                        let location = block.location;

                        // push the scope and remember the stack position
                        self.push_scope();
                        let stack_frame_start = self.stack_height;
                        self.push_instruction(Instruction::FrameBegin(zrust_bytecode::FrameBegin));

                        // evaluate the block and get the result type
                        self.block_expression(block)?;
                        let (result, result_address) =
                            (self.evaluate_unary_operand(true)?, self.stack_last_index());
                        let result_type = result
                            .type_variant(self.scope().borrow().deref())
                            .map_err(|error| Error::Element(location, error))?;
                        self.push_operand(StackElement::Evaluated(result));

                        // pop the scope and get its outputs
                        let scope = self.pop_scope();
                        let outputs = scope.get_ordered_outer_assignments();
                        let outputs_count = outputs.len() + result_type.size();

                        // update the variables in the output scope
                        self.stack_height = stack_frame_start;
                        for (place, assignment) in outputs.into_iter() {
                            self.push_instruction(Instruction::Copy(zrust_bytecode::Copy::new(
                                assignment.address,
                            )));
                            self.scope()
                                .borrow_mut()
                                .update_variable(place, self.stack_last_index())
                                .expect("The variable address always exists as it is checked during the block evaluation");
                        }
                        match result_type {
                            TypeVariant::Unit => {}
                            _ => self.push_instruction(Instruction::Copy(
                                zrust_bytecode::Copy::new(result_address),
                            )),
                        }
                        self.push_instruction(Instruction::FrameEnd(
                            zrust_bytecode::FrameEnd::new(outputs_count),
                        ));
                    }
                    ExpressionOperand::Conditional(conditional) => {
                        self.conditional_expression(conditional)?;
                    }
                    ExpressionOperand::Match(_match) => unimplemented!(),
                    ExpressionOperand::Array(array) => {
                        self.array_expression(array)?;
                    }
                    ExpressionOperand::Tuple(tuple) => {
                        self.tuple_expression(tuple)?;
                    }
                    ExpressionOperand::Structure(structure) => {
                        self.structure_expression(structure)?;
                    }
                }
                match self.pop_operand() {
                    StackElement::Evaluated(element) => element,
                    StackElement::NotEvaluated { .. } => panic!("Is evaluated above"),
                }
            }
            StackElement::Evaluated(element) => element,
        })
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scopes
            .last()
            .cloned()
            .expect("Cannot fail because the global scope has been pushed during initialization")
    }

    fn push_scope(&mut self) {
        self.scopes
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) -> Scope {
        let scope = self
            .scopes
            .pop()
            .expect("Should not be called even when there is only the global scope left");
        Rc::try_unwrap(scope)
            .expect("The innermost scope has no other references")
            .into_inner()
    }

    fn evaluate_unary_operand(&mut self, is_for_stack: bool) -> Result<Element, Error> {
        self.evaluate_operand(is_for_stack)
    }

    fn evaluate_binary_operands(
        &mut self,
        is_for_stack_1: bool,
        is_for_stack_2: bool,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand(is_for_stack_2)?;
        let operand_1 = self.evaluate_operand(is_for_stack_1)?;
        Ok((operand_1, operand_2))
    }

    fn push_operand(&mut self, operand: StackElement) {
        self.operands.push(operand);
    }

    fn pop_operand(&mut self) -> StackElement {
        self.operands
            .pop()
            .expect("Should be called if there are no operands to pop")
    }

    fn push_instruction(&mut self, instruction: Instruction) {
        //        log::trace!(">>> {:03} {:?}", self.instructions.len(), instruction);
        let instruction_ref = &instruction;
        let stack_difference = (dispatch_instruction!(instruction_ref => instruction_ref.outputs_count())
            as isize)
            - (dispatch_instruction!(instruction_ref => instruction_ref.inputs_count()) as isize);
        self.stack_resize(stack_difference);
        self.instructions.push(instruction);
    }

    fn stack_resize(&mut self, difference: isize) {
        if difference > 0 {
            self.stack_height += difference as usize;
        } else {
            self.stack_height -= (-difference) as usize;
        }
    }

    fn stack_last_index(&self) -> usize {
        (self.stack_height - 1) as usize
    }
}
