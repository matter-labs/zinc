//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::error::Error as CompilerError;
use crate::semantic::inference;
use crate::semantic::Array;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
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
use crate::syntax::InnerStatement;
use crate::syntax::IntegerLiteral;
use crate::syntax::OuterStatement;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;
use crate::syntax::TypeVariant;
use crate::CircuitProgram;

pub struct Analyzer {
    scopes: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    instructions: Vec<Instruction>,

    function_addresses: HashMap<usize, usize>,
    function_index: usize,
    is_next_call_instruction: bool,

    stack_position: usize,
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

    const FUNCTION_INDEX_DEBUG: usize = 0;
    const FUNCTION_INDEX_ASSERT: usize = 1;
    const FUNCTION_INDEX_MAIN: usize = 2;
    const FUNCTION_INDEX_NEXT: usize = 3;

    pub fn new(scope: Scope) -> Self {
        Self {
            scopes: {
                let mut scopes = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scopes.push(Rc::new(RefCell::new(scope)));
                scopes
            },
            operands: Vec::with_capacity(Self::STACK_OPERAND_INITIAL_CAPACITY),
            instructions: Vec::with_capacity(Self::VECTOR_INSTRUCTION_INITIAL_CAPACITY),

            function_addresses: HashMap::with_capacity(Self::HASHMAP_FUNCTIONS_INITIAL_CAPACITY),
            is_next_call_instruction: false,

            stack_position: 0,
            function_index: Self::FUNCTION_INDEX_NEXT,
        }
    }

    pub fn compile(mut self, program: CircuitProgram) -> Result<Vec<Instruction>, CompilerError> {
        // insert the placeholders the 'main' function call
        self.instructions
            .push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        self.instructions
            .push(Instruction::NoOperation(zinc_bytecode::NoOperation));

        self.scope()
            .borrow_mut()
            .declare_type(
                "dbg".to_owned(),
                Type::new_function(Self::FUNCTION_INDEX_DEBUG, vec![], Type::Unit),
            )
            .expect(crate::semantic::PANIC_INSTRUCTION_FUNCTION_DECLARATION);

        self.scope()
            .borrow_mut()
            .declare_type(
                "assert".to_owned(),
                Type::new_function(
                    Self::FUNCTION_INDEX_ASSERT,
                    vec![("condition".to_owned(), Type::Boolean)],
                    Type::Unit,
                ),
            )
            .expect(crate::semantic::PANIC_INSTRUCTION_FUNCTION_DECLARATION);

        // compile all the outer statements which generally only declare new items
        for statement in program.statements.into_iter() {
            self.outer_statement(statement)
                .map_err(CompilerError::Semantic)?;
        }

        // replace the placeholders inserted above with an actual 'main' function call
        let main_function_address = self
            .function_addresses
            .get(&Self::FUNCTION_INDEX_MAIN)
            .copied()
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;
        match self
            .scope()
            .borrow()
            .resolve_type("main")
            .expect(crate::semantic::PANIC_RESOLUTION_FUNCTION_MAIN)
        {
            Type::Function {
                arguments,
                return_type,
                ..
            } => {
                self.instructions[0] = Instruction::Call(zinc_bytecode::Call::new(
                    main_function_address,
                    arguments
                        .into_iter()
                        .map(|(_arg_name, arg_type)| arg_type.size())
                        .sum(),
                ));
                self.instructions[1] =
                    Instruction::Exit(zinc_bytecode::Exit::new(return_type.size()));
            }
            _ => panic!(crate::semantic::PANIC_RESOLUTION_FUNCTION_MAIN),
        }

        Ok(self.instructions)
    }

    fn outer_statement(&mut self, statement: OuterStatement) -> Result<(), Error> {
        match statement {
            OuterStatement::Type(statement) => {
                let location = statement.location;
                let r#type = self.resolve_type(statement.r#type.variant)?;
                self.scope()
                    .borrow_mut()
                    .declare_type(statement.identifier.name, r#type)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Struct(statement) => {
                let location = statement.location;

                let type_index = self.allocate_function_index();
                let mut fields = Vec::with_capacity(statement.fields.len());
                for field in statement.fields.into_iter() {
                    fields.push((
                        field.identifier.name,
                        self.resolve_type(field.r#type.variant)?,
                    ));
                }
                let r#type = Type::new_structure(type_index, fields);

                self.scope()
                    .borrow_mut()
                    .declare_type(statement.identifier.name, r#type)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Enum(statement) => {
                let location = statement.location;

                let type_index = self.allocate_function_index();
                let r#type = Type::new_enumeration(
                    type_index,
                    statement
                        .variants
                        .into_iter()
                        .map(|variant| (variant.identifier.name, variant.literal.into()))
                        .collect(),
                );

                self.scope()
                    .borrow_mut()
                    .declare_type(statement.identifier.name, r#type)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Fn(statement) => {
                let location = statement.location;

                let type_index = if statement.identifier.name.as_str() == "main" {
                    Self::FUNCTION_INDEX_MAIN
                } else {
                    self.allocate_function_index()
                };
                let mut arguments = Vec::with_capacity(statement.arguments.len());
                for argument in statement.arguments.iter() {
                    arguments.push((
                        argument.identifier.name.clone(),
                        self.resolve_type(argument.r#type.variant.clone())?,
                    ));
                }
                let return_type = self.resolve_type(statement.return_type.variant.clone())?;
                let r#type = Type::new_function(type_index, arguments, return_type);

                self.scope()
                    .borrow_mut()
                    .declare_type(statement.identifier.name.clone(), r#type)
                    .map_err(|error| Error::Scope(location, error))?;

                // record the function address in the bytecode
                self.function_addresses
                    .insert(type_index, self.instructions.len());

                // reset the stack frame address counter
                self.stack_position = 0;

                // start a new scope and declare the function arguments there
                self.push_scope();
                for argument in statement.arguments.into_iter() {
                    let r#type = self.resolve_type(argument.r#type.variant)?;
                    let address = self.allocate_stack_space(r#type.size());
                    self.scope()
                        .borrow_mut()
                        .declare_variable(argument.identifier.name, r#type, false, address)
                        .map_err(|error| Error::Scope(location, error))?;
                }

                // compile the function block
                let result = self.block_expression(statement.body)?;
                let return_type = result
                    .r#type(self.scope().borrow().deref())
                    .map_err(|error| Error::Element(location, error))?;
                self.push_operand(StackElement::Evaluated(result));
                self.pop_scope();

                // check the function return type to match the block result
                let expected_type = self.resolve_type(statement.return_type.variant)?;
                if expected_type != return_type {
                    return Err(Error::FunctionReturnTypeMismatch(
                        statement.return_type.location,
                        type_index,
                        expected_type,
                        return_type,
                    ));
                }

                self.instructions
                    .push(Instruction::Return(zinc_bytecode::Return::new(
                        expected_type.size(),
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
                let rvalue = self.expression(r#let.expression, true)?;

                let r#type = if let Some(r#type) = r#let.r#type {
                    // get and resolve the type
                    let let_type = match r#type.variant {
                        TypeVariant::Alias { path } => {
                            let location = r#type.location;
                            match self.expression(path, false)? {
                                Element::Type(r#type) => r#type,
                                element => return Err(Error::ExpectedType(location, element)),
                            }
                        }
                        type_variant => self.resolve_type(type_variant)?,
                    };

                    let (is_signed, bitlength) = rvalue
                        .cast(
                            Element::Type(let_type.clone()),
                            self.scope().borrow().deref(),
                        )
                        .map_err(|error| Error::Element(location, error))?;
                    self.instructions
                        .push(Instruction::Cast(zinc_bytecode::Cast::new(
                            is_signed,
                            bitlength as u8,
                        )));
                    let_type
                } else {
                    rvalue
                        .r#type(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(location, error))?
                };

                let size = r#type.size();
                let address = self.allocate_stack_space(size);
                match size {
                    0 => {}
                    1 => {
                        self.instructions
                            .push(Instruction::PopStore(zinc_bytecode::PopStore::new(address)));
                    }
                    size => {
                        self.instructions.push(Instruction::PopStoreArray(
                            zinc_bytecode::PopStoreArray::new(address, size),
                        ));
                    }
                }
                self.scope()
                    .borrow_mut()
                    .declare_variable(r#let.identifier.name, r#type, r#let.is_mutable, address)
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
                let index = Constant::new_range_bound(range_start, range_bitlength);
                let index_address = self.allocate_stack_space(index.r#type().size());
                self.instructions.push(Instruction::PushConst(index.into()));
                self.instructions
                    .push(Instruction::PopStore(zinc_bytecode::PopStore::new(
                        index_address,
                    )));
                self.instructions
                    .push(Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(
                        iterations_count,
                    )));

                // declare the index variable
                self.push_scope();
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        r#loop.index_identifier.name,
                        Type::new_integer_unsigned(range_bitlength),
                        false,
                        index_address,
                    )
                    .map_err(|error| Error::Scope(location, error))?;
                self.block_expression(r#loop.block)?;

                self.instructions.push(Instruction::PushConst(
                    Constant::new_one(range_bitlength).into(),
                ));
                self.instructions
                    .push(Instruction::LoadPush(zinc_bytecode::LoadPush::new(
                        index_address,
                    )));
                self.instructions.push(if is_reverse {
                    Instruction::Sub(zinc_bytecode::Sub)
                } else {
                    Instruction::Add(zinc_bytecode::Add)
                });
                self.instructions
                    .push(Instruction::PopStore(zinc_bytecode::PopStore::new(
                        index_address,
                    )));
                self.instructions
                    .push(Instruction::LoopEnd(zinc_bytecode::LoopEnd));

                self.pop_scope();
            }
            InnerStatement::Expression(expression) => {
                self.expression(expression, true)?;
            }
        }

        Ok(())
    }

    fn expression(&mut self, expression: Expression, is_rvalue: bool) -> Result<Element, Error> {
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
                        .check_assignment(&place)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    let address = self
                        .scope()
                        .borrow_mut()
                        .get_variable_address(&place)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    self.instructions
                        .push(Instruction::PopStore(zinc_bytecode::PopStore::new(address)));
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE)
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE)
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Or(zinc_bytecode::Or));

                    let result = operand_1
                        .or(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Xor(zinc_bytecode::Xor));

                    let result = operand_1
                        .xor(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::And(zinc_bytecode::And));

                    let result = operand_1
                        .and(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Eq(zinc_bytecode::Eq));

                    let result = operand_1
                        .equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Ne(zinc_bytecode::Ne));

                    let result = operand_1
                        .not_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Ge(zinc_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Le(zinc_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Gt(zinc_bytecode::Gt));

                    let result = operand_1
                        .greater(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Lt(zinc_bytecode::Lt));

                    let result = operand_1
                        .lesser(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Add(zinc_bytecode::Add));

                    let result = operand_1
                        .add(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Sub(zinc_bytecode::Sub));

                    let result = operand_1
                        .subtract(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Mul(zinc_bytecode::Mul));

                    let result = operand_1
                        .multiply(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Div(zinc_bytecode::Div));

                    let result = operand_1
                        .divide(operand_2, self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self.evaluate_binary_operands(true, true)?;
                    self.instructions.push(Instruction::Rem(zinc_bytecode::Rem));

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
                    self.instructions
                        .push(Instruction::Cast(zinc_bytecode::Cast::new(
                            is_signed,
                            bitlength as u8,
                        )));
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                        Type::new_integer(is_signed, bitlength),
                    ))));
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    self.instructions.push(Instruction::Neg(zinc_bytecode::Neg));

                    let result = operand_1
                        .negate(self.scope().borrow().deref())
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.evaluate_unary_operand(true)?;
                    self.instructions.push(Instruction::Not(zinc_bytecode::Not));

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
                ExpressionObject::Operator(ExpressionOperator::InstructionCall) => {
                    self.is_next_call_instruction = true;
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let stack_frame_start = self.stack_position;

                    let (operand_1, operand_2) = self.evaluate_binary_operands(false, false)?;

                    // check if the first operand is a function and get its data
                    let (function_index, argument_types, return_type) = match operand_1 {
                        Element::Type(Type::Function {
                            index,
                            arguments,
                            return_type,
                        }) => (index, arguments, return_type),
                        operand => {
                            return Err(Error::FunctionCallOnNotCallable(element.location, operand))
                        }
                    };

                    // check the number of the arguments
                    let argument_values = match operand_2 {
                        Element::ValueList(values) => values,
                        _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
                    };
                    let argument_types_count = argument_types.len();
                    let argument_values_count = argument_values.len();

                    // check if the call is a direct instruction call like 'dbg' or 'assert'
                    let is_instruction = self.is_next_call_instruction;
                    self.is_next_call_instruction = false;

                    if !is_instruction && function_index != Self::FUNCTION_INDEX_DEBUG {
                        if argument_values.len() != argument_types.len() {
                            return Err(Error::FunctionArgumentCountMismatch(
                                element.location,
                                function_index,
                                argument_types_count,
                                argument_values_count,
                            ));
                        }

                        // check the argument types
                        for (argument_index, (argument_name, expected_type)) in
                            argument_types.into_iter().enumerate()
                        {
                            let actual_type = argument_values[argument_index].r#type();
                            if expected_type != actual_type {
                                return Err(Error::FunctionArgumentTypeMismatch(
                                    element.location,
                                    function_index,
                                    argument_name,
                                    expected_type,
                                    actual_type,
                                ));
                            }
                        }
                    }

                    if is_instruction {
                        let instruction = match function_index {
                            Self::FUNCTION_INDEX_DEBUG => {
                                Instruction::Log(zinc_bytecode::Dbg::new(
                                    "DEBUG".to_owned(), // TODO: pass an actual string
                                    argument_values_count,
                                ))
                            }
                            Self::FUNCTION_INDEX_ASSERT => {
                                Instruction::Assert(zinc_bytecode::Assert)
                            }
                            function_index => {
                                return Err(Error::FunctionInstructionUnknown(
                                    element.location,
                                    function_index,
                                ))
                            }
                        };
                        self.instructions.push(instruction);
                    } else {
                        let function_address = self
                            .function_addresses
                            .get(&function_index)
                            .copied()
                            .unwrap();

                        self.instructions
                            .push(Instruction::Call(zinc_bytecode::Call::new(
                                function_address,
                                argument_values_count,
                            )));
                    }

                    self.stack_position = stack_frame_start;

                    self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                        *return_type,
                    ))))
                }
                ExpressionObject::Operator(ExpressionOperator::Path) => unimplemented!(),
            }
        }

        self.evaluate_operand(is_rvalue)
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let constant = Constant::from(literal);
        self.instructions
            .push(Instruction::PushConst(constant.clone().into()));
        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let constant = Constant::try_from(literal)
            .map_err(|error| Error::ConstantTypeInference(location, error))?;
        self.instructions
            .push(Instruction::PushConst(constant.clone().into()));
        Ok(Element::Constant(constant))
    }

    fn string_literal(&mut self, _literal: StringLiteral) -> Result<Element, Error> {
        unimplemented!();
    }

    fn identifier(&mut self, identifier: Identifier, is_rvalue: bool) -> Result<Element, Error> {
        let location = identifier.location;

        if is_rvalue {
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
                .map(|declaration| Value::new(declaration.r#type))
                .map_err(|error| Error::Scope(location, error))?;
            self.instructions
                .push(Instruction::LoadPush(zinc_bytecode::LoadPush::new(address)));
            Ok(Element::Value(value))
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
                    Ok(Element::Place(place))
                }
                ScopeItem::Type => {
                    let r#type = self
                        .scope()
                        .borrow()
                        .resolve_type(&identifier.name)
                        .map_err(|error| Error::Scope(location, error))?;
                    Ok(Element::Type(r#type))
                }
                ScopeItem::Variant => unimplemented!(),
            }
        }
    }

    fn r#type(&mut self, r#type: syntax::Type) -> Result<Element, Error> {
        self.resolve_type(r#type.variant).map(Element::Type)
    }

    fn block_expression(&mut self, block: BlockExpression) -> Result<Element, Error> {
        for statement in block.statements.into_iter() {
            self.inner_statement(statement)?;
        }
        if let Some(expression) = block.expression {
            self.expression(*expression, true)
        } else {
            Ok(Element::Value(Value::Unit))
        }
    }

    fn conditional_expression(
        &mut self,
        conditional: ConditionalExpression,
    ) -> Result<Element, Error> {
        let location = conditional.location;
        let condition_location = conditional.condition.location;

        // compile the condition and check if it is boolean
        let condition_result = self.expression(*conditional.condition, true)?;
        match condition_result
            .r#type(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?
        {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanExpression(
                    condition_location,
                    r#type,
                ))
            }
        }

        self.instructions.push(Instruction::If(zinc_bytecode::If));

        self.push_scope();
        let main_result = self.block_expression(conditional.main_block)?;
        let main_type = main_result
            .r#type(self.scope().borrow().deref())
            .map_err(|error| Error::Element(location, error))?;
        self.pop_scope();

        let else_type = if let Some(else_block) = conditional.else_block {
            self.instructions
                .push(Instruction::Else(zinc_bytecode::Else));

            self.push_scope();
            let else_result = self.block_expression(else_block)?;
            let else_type = else_result
                .r#type(self.scope().borrow().deref())
                .map_err(|error| Error::Element(location, error))?;
            self.pop_scope();

            else_type
        } else {
            Type::Unit
        };

        self.instructions
            .push(Instruction::EndIf(zinc_bytecode::EndIf));

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypeMismatch(
                location, main_type, else_type,
            ));
        }

        Ok(main_result)
    }

    fn array_expression(&mut self, array: ArrayExpression) -> Result<Element, Error> {
        let location = array.location;

        let mut result = Array::default();
        for expression in array.elements.into_iter() {
            let value = match self.expression(expression, true)? {
                Element::Value(value) => value,
                Element::Constant(constant) => Value::new(constant.r#type()),
                _ => panic!(crate::semantic::PANIC_ALWAYS_EVALUATED),
            };
            result
                .push(value.r#type())
                .map_err(|error| Error::ArrayLiteral(location, error))?;
        }

        Ok(Element::Value(Value::Array(result)))
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = Tuple::default();
        for expression in tuple.elements.into_iter() {
            let value = match self.expression(expression, true)? {
                Element::Value(value) => value,
                Element::Constant(constant) => Value::new(constant.r#type()),
                _ => panic!(crate::semantic::PANIC_ALWAYS_EVALUATED),
            };
            result.push(value.r#type());
        }

        Ok(Element::Value(Value::Tuple(result)))
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<Element, Error> {
        let location = structure.location;

        let mut result = Structure::default();
        for (identifier, expression) in structure.fields.into_iter() {
            let value = match self.expression(expression, true)? {
                Element::Value(value) => value,
                Element::Constant(constant) => Value::new(constant.r#type()),
                _ => panic!(crate::semantic::PANIC_ALWAYS_EVALUATED),
            };
            result
                .push(identifier.name, value.r#type())
                .map_err(|error| Error::StructureLiteral(location, error))?;
        }

        Ok(Element::Value(Value::Structure(result)))
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<Element, Error> {
        let mut values = Vec::with_capacity(list.len());
        for expression in list.into_iter() {
            let size = match self.expression(expression, true)? {
                Element::Value(value) => {
                    let size = value.r#type().size();
                    values.push(value);
                    size
                }
                Element::Constant(constant) => {
                    let r#type = constant.r#type();
                    let size = r#type.size();
                    values.push(Value::new(r#type));
                    size
                }
                _ => panic!(crate::semantic::PANIC_ALWAYS_EVALUATED),
            };
            let argument_address = self.allocate_stack_space(size);
            self.instructions
                .push(Instruction::PopStore(zinc_bytecode::PopStore::new(
                    argument_address,
                )));
        }
        Ok(Element::ValueList(values))
    }

    fn evaluate_operand(&mut self, is_for_stack: bool) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Value(Value::Unit)),
                ExpressionOperand::BooleanLiteral(literal) => self.boolean_literal(literal),
                ExpressionOperand::IntegerLiteral(literal) => self.integer_literal(literal),
                ExpressionOperand::StringLiteral(literal) => self.string_literal(literal),
                ExpressionOperand::Identifier(identifier) => {
                    self.identifier(identifier, is_for_stack)
                }
                ExpressionOperand::List(expressions) => self.list_expression(expressions),
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
                ExpressionOperand::Match(_match) => unimplemented!(),
                ExpressionOperand::Array(expression) => self.array_expression(expression),
                ExpressionOperand::Tuple(expression) => self.tuple_expression(expression),
                ExpressionOperand::Structure(expression) => self.structure_expression(expression),
            },
            StackElement::Evaluated(element) => Ok(element),
        }
    }

    fn resolve_type(&mut self, type_variant: TypeVariant) -> Result<Type, Error> {
        Ok(match type_variant {
            TypeVariant::Unit => Type::Unit,
            TypeVariant::Boolean => Type::Boolean,
            TypeVariant::IntegerUnsigned { bitlength } => Type::IntegerUnsigned { bitlength },
            TypeVariant::IntegerSigned { bitlength } => Type::IntegerSigned { bitlength },
            TypeVariant::Field => Type::Field,
            TypeVariant::Array { type_variant, size } => Type::Array {
                r#type: self.resolve_type(*type_variant).map(Box::new)?,
                size: size.into(),
            },
            TypeVariant::Tuple { type_variants } => {
                let mut types = Vec::with_capacity(type_variants.len());
                for type_variant in type_variants.into_iter() {
                    types.push(self.resolve_type(type_variant)?);
                }
                Type::Tuple { types }
            }
            TypeVariant::Alias { path } => {
                let location = path.location;
                match self.expression(path, false)? {
                    Element::Type(r#type) => r#type,
                    element => return Err(Error::ExpectedType(location, element)),
                }
            }
        })
    }

    fn scope(&self) -> Rc<RefCell<Scope>> {
        self.scopes
            .last()
            .cloned()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE)
    }

    fn push_scope(&mut self) {
        self.scopes
            .push(Rc::new(RefCell::new(Scope::new(Some(self.scope())))));
    }

    fn pop_scope(&mut self) {
        self.scopes
            .pop()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_A_SCOPE);
    }

    fn evaluate_unary_operand(&mut self, is_rvalue: bool) -> Result<Element, Error> {
        self.evaluate_operand(is_rvalue)
    }

    fn evaluate_binary_operands(
        &mut self,
        is_rvalue_1: bool,
        is_rvalue_2: bool,
    ) -> Result<(Element, Element), Error> {
        let operand_2 = self.evaluate_operand(is_rvalue_2)?;
        let operand_1 = self.evaluate_operand(is_rvalue_1)?;
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

    fn allocate_function_index(&mut self) -> usize {
        self.function_index += 1;
        self.function_index - 1
    }

    fn allocate_stack_space(&mut self, size: usize) -> usize {
        self.stack_position += size;
        self.stack_position - size
    }
}
