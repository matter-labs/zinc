//!
//! The semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::error::Error as CompilerError;
use crate::semantic::scope::VariableItem;
use crate::semantic::Array;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::IntegerConstant;
use crate::semantic::Place;
use crate::semantic::Scope;
use crate::semantic::ScopeItem;
use crate::semantic::ScopeStaticItem;
use crate::semantic::ScopeVariableItem;
use crate::semantic::Structure;
use crate::semantic::Tuple;
use crate::semantic::Type;
use crate::semantic::Value;
use crate::syntax;
use crate::syntax::ArrayExpression;
use crate::syntax::BlockExpression;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpression;
use crate::syntax::ConstStatement;
use crate::syntax::Expression;
use crate::syntax::ExpressionObject;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::Identifier;
use crate::syntax::InnerStatement;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpression;
use crate::syntax::MemberInteger;
use crate::syntax::MemberString;
use crate::syntax::OuterStatement;
use crate::syntax::PatternVariant;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpression;
use crate::syntax::TupleExpression;
use crate::syntax::TypeVariant;
use crate::CircuitProgram;

pub struct BinaryAnalyzer {
    scopes: Vec<Rc<RefCell<Scope>>>,
    operands: Vec<StackElement>,
    instructions: Vec<Instruction>,

    function_addresses: HashMap<String, usize>,
    is_next_call_instruction: bool,

    stack_position: usize,
}

#[derive(Debug, Clone)]
enum StackElement {
    NotEvaluated(ExpressionOperand),
    Evaluated(Element),
}

enum ResolutionMode {
    Member,
    Place,
    Value,
}

impl Default for BinaryAnalyzer {
    fn default() -> Self {
        Self::new(Scope::default())
    }
}

impl BinaryAnalyzer {
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

            function_addresses: HashMap::with_capacity(Self::HASHMAP_FUNCTIONS_INITIAL_CAPACITY),
            is_next_call_instruction: false,

            stack_position: 0,
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
                Type::new_function(
                    "dbg".to_owned(),
                    vec![("format".to_owned(), Type::String)],
                    Type::Unit,
                ),
            )
            .expect(crate::semantic::PANIC_INSTRUCTION_FUNCTION_DECLARATION);

        self.scope()
            .borrow_mut()
            .declare_type(
                "assert".to_owned(),
                Type::new_function(
                    "assert".to_owned(),
                    vec![
                        ("condition".to_owned(), Type::Boolean),
                        ("message".to_owned(), Type::String),
                    ],
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
            .get("main")
            .copied()
            .ok_or(Error::FunctionMainMissing)
            .map_err(CompilerError::Semantic)?;
        match self
            .scope()
            .borrow()
            .get_item("main")
            .expect(crate::semantic::PANIC_RESOLUTION_FUNCTION_MAIN)
        {
            ScopeItem::Type(Type::Function {
                arguments,
                return_type,
                ..
            }) => {
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
            OuterStatement::Const(statement) => self.const_statement(statement)?,
            OuterStatement::Static(statement) => {
                let location = statement.location;

                // compile the expression being assigned
                let rvalue = self.expression(statement.expression, ResolutionMode::Value)?;

                let const_type = match statement.r#type.variant {
                    TypeVariant::Alias { path } => {
                        let location = path.location;
                        match self.expression(path, ResolutionMode::Place)? {
                            Element::Type(r#type) => r#type,
                            element => return Err(Error::ExpectedType(location, element)),
                        }
                    }
                    type_variant => self.resolve_type(type_variant)?,
                };
                let cast_result = rvalue
                    .cast(&Element::Type(const_type))
                    .map_err(|error| Error::Element(location, error))?;
                let constant = match rvalue {
                    Element::Constant(Constant::Integer(mut constant)) => {
                        if let Some(cast_result) = cast_result {
                            constant.is_signed = cast_result.0;
                            constant.bitlength = cast_result.1;
                        }
                        Constant::Integer(constant)
                    }
                    element => return Err(Error::ExpressionIsNotConstant(location, element)),
                };
                let size = constant.r#type().size();
                let address = self.allocate_stack_space(size);

                self.scope()
                    .borrow_mut()
                    .declare_static(
                        statement.identifier.name,
                        ScopeStaticItem::new(constant, address),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
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

                let mut fields = Vec::with_capacity(statement.fields.len());
                for field in statement.fields.into_iter() {
                    fields.push((
                        field.identifier.name,
                        self.resolve_type(field.r#type.variant)?,
                    ));
                }
                let r#type = Type::new_structure(statement.identifier.name.clone(), fields);

                self.scope()
                    .borrow_mut()
                    .declare_type(statement.identifier.name, r#type)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Enum(statement) => {
                let location = statement.location;

                let r#type = Type::new_enumeration(
                    statement.identifier.name.clone(),
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

                let identifier = statement.identifier.name;

                let mut arguments = Vec::with_capacity(statement.arguments.len());
                for argument in statement.arguments.iter() {
                    arguments.push((
                        argument.identifier.name.clone(),
                        self.resolve_type(argument.r#type.variant.clone())?,
                    ));
                }
                let return_type = self.resolve_type(statement.return_type.variant.clone())?;
                let r#type = Type::new_function(identifier.clone(), arguments, return_type);

                self.scope()
                    .borrow_mut()
                    .declare_type(identifier.clone(), r#type)
                    .map_err(|error| Error::Scope(location, error))?;

                // record the function address in the bytecode
                self.function_addresses
                    .insert(identifier.clone(), self.instructions.len());

                // reset the stack frame address counter
                self.stack_position = 0;

                // start a new scope and declare the function arguments there
                self.push_scope();
                for argument in statement.arguments.into_iter() {
                    let r#type = self.resolve_type(argument.r#type.variant)?;
                    let address = self.allocate_stack_space(r#type.size());
                    self.scope()
                        .borrow_mut()
                        .declare_variable(
                            argument.identifier.name,
                            ScopeVariableItem::new(r#type, false, address),
                        )
                        .map_err(|error| Error::Scope(location, error))?;
                }

                // compile the function block
                let result = self.block_expression(statement.body)?;
                let return_type = self.element_type(&result)?;
                self.push_operand(StackElement::Evaluated(result));
                self.pop_scope();

                // check the function return type to match the block result
                let expected_type = self.resolve_type(statement.return_type.variant)?;
                if expected_type != return_type {
                    return Err(Error::FunctionReturnTypeMismatch(
                        statement.return_type.location,
                        identifier,
                        expected_type,
                        return_type,
                    ));
                }

                self.instructions
                    .push(Instruction::Return(zinc_bytecode::Return::new(
                        expected_type.size(),
                    )));
            }
            OuterStatement::Mod(statement) => {
                let location = statement.location;
                self.scope()
                    .borrow_mut()
                    .declare_module(statement.identifier.name, Scope::new(None))
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Use(_statement) => {}
        }

        Ok(())
    }

    fn inner_statement(&mut self, statement: InnerStatement) -> Result<(), Error> {
        match statement {
            InnerStatement::Let(statement) => {
                let location = statement.location;

                // compile the expression being assigned
                let rvalue = self.expression(statement.expression, ResolutionMode::Value)?;

                let r#type = if let Some(r#type) = statement.r#type {
                    // get and resolve the type
                    let let_type = match r#type.variant {
                        TypeVariant::Alias { path } => {
                            let location = r#type.location;
                            match self.expression(path, ResolutionMode::Place)? {
                                Element::Type(r#type) => r#type,
                                element => return Err(Error::ExpectedType(location, element)),
                            }
                        }
                        type_variant => self.resolve_type(type_variant)?,
                    };

                    if let Some((is_signed, bitlength)) = rvalue
                        .cast(&Element::Type(let_type.clone()))
                        .map_err(|error| Error::Element(location, error))?
                    {
                        self.instructions
                            .push(Instruction::Cast(zinc_bytecode::Cast::new(
                                is_signed,
                                bitlength as u8,
                            )));
                    }
                    let_type
                } else {
                    self.element_type(&rvalue)?
                };

                let size = r#type.size();
                let address = self.allocate_stack_space(size);
                self.add_instruction_pop_store(address, size);
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        statement.identifier.name,
                        ScopeVariableItem::new(r#type, statement.is_mutable, address),
                    )
                    .map_err(|error| Error::Scope(location, error))?;
            }
            InnerStatement::Const(statement) => self.const_statement(statement)?,
            InnerStatement::Loop(statement) => {
                let location = statement.location;

                // infer the bitlength of the range start and end
                let range_bitlength = IntegerConstant::infer_enough_bitlength(&[
                    &statement.range_start,
                    &statement.range_end,
                ])
                .map_err(|error| Error::TypeInferenceLoopBounds(location, error))?;

                // calculate the iterations number and if the loop is reverse
                let range_start: usize = statement.range_start.into();
                let range_end: usize = statement.range_end.into();
                let iterations_count = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if statement.is_range_inclusive { 1 } else { 0 };
                let is_reverse = range_start > range_end;

                // create the index value and get its address
                let index = IntegerConstant::new_range_bound(range_start, range_bitlength);
                let index_address = self.allocate_stack_space(index.r#type().size());
                self.instructions.push(index.into());
                self.instructions
                    .push(Instruction::PopStore(zinc_bytecode::PopStore::new(
                        index_address,
                    )));

                // create the while allowed condition
                let while_allowed_address = match statement.while_condition {
                    Some(_) => {
                        let while_allowed = Constant::Boolean(true);
                        let while_allowed_address =
                            self.allocate_stack_space(while_allowed.r#type().size());
                        self.instructions.push(while_allowed.into());
                        self.instructions.push(Instruction::PopStore(
                            zinc_bytecode::PopStore::new(while_allowed_address),
                        ));
                        Some(while_allowed_address)
                    }
                    None => None,
                };

                self.instructions
                    .push(Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(
                        iterations_count,
                    )));

                // declare the index variable
                self.push_scope();
                self.scope()
                    .borrow_mut()
                    .declare_variable(
                        statement.index_identifier.name,
                        ScopeVariableItem::new(
                            Type::new_integer_unsigned(range_bitlength),
                            false,
                            index_address,
                        ),
                    )
                    .map_err(|error| Error::Scope(location, error))?;

                // check the while condition, set the allowed variable, and execute the loop body
                if let (Some(expression), Some(while_allowed_address)) =
                    (statement.while_condition, while_allowed_address)
                {
                    let location = expression.location;
                    let while_result = self.expression(expression, ResolutionMode::Value)?;
                    match self.element_type(&while_result)? {
                        Type::Boolean => {}
                        r#type => {
                            return Err(Error::LoopWhileExpectedBooleanCondition(location, r#type))
                        }
                    }

                    self.instructions.push(Instruction::Not(zinc_bytecode::Not));
                    self.instructions.push(Instruction::If(zinc_bytecode::If));
                    self.instructions.push(Constant::Boolean(false).into());
                    self.add_instruction_pop_store(
                        while_allowed_address,
                        Type::new_boolean().size(),
                    );
                    self.instructions
                        .push(Instruction::EndIf(zinc_bytecode::EndIf));

                    self.add_instruction_load_push(
                        while_allowed_address,
                        Type::new_boolean().size(),
                    );
                    self.instructions.push(Instruction::If(zinc_bytecode::If));
                    self.block_expression(statement.block)?;
                    self.instructions
                        .push(Instruction::EndIf(zinc_bytecode::EndIf));
                } else {
                    self.block_expression(statement.block)?;
                }

                // increment the loop counter
                self.instructions
                    .push(IntegerConstant::new_one(range_bitlength).into());
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
                self.expression(expression, ResolutionMode::Value)?;
            }
        }

        Ok(())
    }

    fn const_statement(&mut self, statement: ConstStatement) -> Result<(), Error> {
        let location = statement.location;
        let expression_location = statement.expression.location;

        // compile the expression being assigned
        let rvalue = self.expression(statement.expression, ResolutionMode::Value)?;

        let const_type = match statement.r#type.variant {
            TypeVariant::Alias { path } => {
                let location = path.location;
                match self.expression(path, ResolutionMode::Place)? {
                    Element::Type(r#type) => r#type,
                    element => return Err(Error::ExpectedType(location, element)),
                }
            }
            type_variant => self.resolve_type(type_variant)?,
        };
        let cast_result = rvalue
            .cast(&Element::Type(const_type))
            .map_err(|error| Error::Element(location, error))?;
        let constant = match rvalue {
            Element::Constant(Constant::Integer(mut constant)) => {
                if let Some(cast_result) = cast_result {
                    constant.is_signed = cast_result.0;
                    constant.bitlength = cast_result.1;
                }
                Constant::Integer(constant)
            }
            element => return Err(Error::ExpressionIsNotConstant(expression_location, element)),
        };

        self.scope()
            .borrow_mut()
            .declare_constant(statement.identifier.name, constant)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn expression(
        &mut self,
        expression: Expression,
        resolution_mode: ResolutionMode,
    ) -> Result<Element, Error> {
        let location = expression.location;
        for element in expression.into_iter() {
            match element.object {
                ExpressionObject::Operand(operand) => {
                    self.push_operand(StackElement::NotEvaluated(operand))
                }
                ExpressionObject::Operator(ExpressionOperator::Assignment) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Place, ResolutionMode::Value)?;
                    let place = operand_1
                        .assign(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    let r#type = self.element_type(&operand_2)?;

                    let variable = self
                        .scope()
                        .borrow_mut()
                        .get_variable(&place)
                        .map_err(|error| Error::Scope(element.location, error))?;
                    if !variable.is_mutable {
                        return Err(Error::AssignmentToImmutableMemory(location, place));
                    }
                    let size = variable.r#type.size();
                    if variable.r#type != r#type {
                        return Err(Error::AssignmentTypesMismatch(
                            location,
                            r#type,
                            place,
                            variable.r#type,
                        ));
                    }

                    self.add_instruction_pop_store(variable.address, size);
                    self.push_operand(StackElement::Evaluated(Element::Value(Value::Unit)));
                }
                ExpressionObject::Operator(ExpressionOperator::Range) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE);
                }
                ExpressionObject::Operator(ExpressionOperator::RangeInclusive) => {
                    panic!(crate::semantic::PANIC_RANGE_OPERATORS_ARE_UNAVAILABLE);
                }
                ExpressionObject::Operator(ExpressionOperator::Or) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Or(zinc_bytecode::Or));

                    let result = operand_1
                        .or(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Xor) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Xor(zinc_bytecode::Xor));

                    let result = operand_1
                        .xor(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::And) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::And(zinc_bytecode::And));

                    let result = operand_1
                        .and(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Equals) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Eq(zinc_bytecode::Eq));

                    let result = operand_1
                        .equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::NotEquals) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Ne(zinc_bytecode::Ne));

                    let result = operand_1
                        .not_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::GreaterEquals) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Ge(zinc_bytecode::Ge));

                    let result = operand_1
                        .greater_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::LesserEquals) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Le(zinc_bytecode::Le));

                    let result = operand_1
                        .lesser_equals(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Greater) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Gt(zinc_bytecode::Gt));

                    let result = operand_1
                        .greater(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Lesser) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Lt(zinc_bytecode::Lt));

                    let result = operand_1
                        .lesser(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Addition) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Add(zinc_bytecode::Add));

                    let result = operand_1
                        .add(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Subtraction) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Sub(zinc_bytecode::Sub));

                    let result = operand_1
                        .subtract(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Multiplication) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Mul(zinc_bytecode::Mul));

                    let result = operand_1
                        .multiply(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Division) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Div(zinc_bytecode::Div));

                    let result = operand_1
                        .divide(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Remainder) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Rem(zinc_bytecode::Rem));

                    let result = operand_1
                        .remainder(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Casting) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Value, ResolutionMode::Place)?;
                    if let Some((is_signed, bitlength)) = operand_1
                        .cast(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?
                    {
                        self.instructions
                            .push(Instruction::Cast(zinc_bytecode::Cast::new(
                                is_signed,
                                bitlength as u8,
                            )));
                        self.push_operand(StackElement::Evaluated(Element::Value(Value::new(
                            Type::new_integer(is_signed, bitlength),
                        ))));
                    } else {
                        self.push_operand(StackElement::Evaluated(operand_1));
                    }
                }
                ExpressionObject::Operator(ExpressionOperator::Negation) => {
                    let operand_1 = self.evaluate_unary_operand(ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Neg(zinc_bytecode::Neg));

                    let result = operand_1
                        .negate()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Not) => {
                    let operand_1 = self.evaluate_unary_operand(ResolutionMode::Value)?;
                    self.instructions.push(Instruction::Not(zinc_bytecode::Not));

                    let result = operand_1
                        .not()
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(result));
                }
                ExpressionObject::Operator(ExpressionOperator::Indexing) => {
                    let (mut operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Place, ResolutionMode::Place)?;

                    operand_1
                        .index(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::Field) => {
                    let (mut operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Place, ResolutionMode::Place)?;

                    operand_1
                        .field(&operand_2)
                        .map_err(|error| Error::Element(element.location, error))?;
                    self.push_operand(StackElement::Evaluated(operand_1));
                }
                ExpressionObject::Operator(ExpressionOperator::InstructionCall) => {
                    self.is_next_call_instruction = true;
                }
                ExpressionObject::Operator(ExpressionOperator::Call) => {
                    let stack_frame_start = self.stack_position;

                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Place, ResolutionMode::Place)?;

                    // check if the first operand is a function and get its data
                    let (identifier, argument_types, return_type) = match operand_1 {
                        Element::Type(Type::Function {
                            identifier,
                            arguments,
                            return_type,
                        }) => (identifier, arguments, return_type),
                        operand => {
                            return Err(Error::FunctionCallNotCallableObject(
                                element.location,
                                operand,
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

                    // check if the call is a direct instruction call like 'dbg' or 'assert'
                    let is_instruction = self.is_next_call_instruction;
                    self.is_next_call_instruction = false;

                    if !is_instruction && identifier.as_str() != "dbg" {
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
                                self.element_type(&argument_values[argument_index])?;
                            if expected_type != actual_type {
                                return Err(Error::FunctionArgumentTypeMismatch(
                                    element.location,
                                    identifier,
                                    argument_name,
                                    expected_type,
                                    actual_type,
                                ));
                            }
                        }
                    }

                    if is_instruction {
                        let instruction = match identifier.as_str() {
                            "dbg" => {
                                let string = match &argument_values[0] {
                                    Element::Constant(Constant::String(string)) => {
                                        string.to_owned()
                                    }
                                    argument => {
                                        return Err(Error::InstructionDebugExpectedString(
                                            element.location,
                                            argument.to_owned(),
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
                        self.instructions.push(instruction);
                    } else {
                        let function_address = self
                            .function_addresses
                            .get(identifier.as_str())
                            .copied()
                            .expect(crate::semantic::PANIC_FUNCTION_ADDRESS_ALWAYS_EXISTS);

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
                ExpressionObject::Operator(ExpressionOperator::Path) => {
                    let (operand_1, operand_2) = self
                        .evaluate_binary_operands(ResolutionMode::Place, ResolutionMode::Member)?;

                    let variant_name = match operand_2 {
                        Element::Place(place) => place.identifier,
                        Element::MemberString(member) => member,
                        operand => {
                            return Err(Error::PathOperatorSecondOperandExpectedStringConstant(
                                element.location,
                                operand,
                            ))
                        }
                    };

                    match operand_1 {
                        Element::Type(Type::Enumeration { identifier, .. }) => {
                            let value = self
                                .scope()
                                .borrow()
                                .get_variant(&identifier, &variant_name)
                                .map_err(|error| Error::Scope(element.location, error))?;
                            self.operands
                                .push(StackElement::Evaluated(Element::Constant(Constant::from(
                                    (value, crate::BITLENGTH_BYTE),
                                ))));
                        }
                        Element::Module(_name) => unimplemented!(),
                        operand => {
                            return Err(Error::PathOperatorFirstOperandExpectedNamespace(
                                element.location,
                                operand,
                            ))
                        }
                    }
                }
            }
        }

        self.evaluate_operand(resolution_mode)
    }

    fn boolean_literal(&mut self, literal: BooleanLiteral) -> Result<Element, Error> {
        let constant = Constant::from(literal);
        self.instructions.push(constant.clone().into());
        Ok(Element::Constant(constant))
    }

    fn integer_literal(&mut self, literal: IntegerLiteral) -> Result<Element, Error> {
        let location = literal.location;

        let integer = IntegerConstant::try_from(&literal)
            .map_err(|error| Error::TypeInferenceConstant(location, error))?;
        self.instructions.push(integer.clone().into());
        Ok(Element::Constant(Constant::Integer(integer)))
    }

    fn string_literal(&mut self, literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }

    fn member_integer(&mut self, integer: MemberInteger) -> Result<Element, Error> {
        let integer = integer.literal.into();
        Ok(Element::MemberInteger(integer))
    }

    fn member_string(&mut self, member_name: MemberString) -> Result<Element, Error> {
        Ok(Element::MemberString(member_name.name))
    }

    fn identifier(
        &mut self,
        identifier: Identifier,
        resolution_mode: ResolutionMode,
    ) -> Result<Element, Error> {
        let location = identifier.location;

        match resolution_mode {
            ResolutionMode::Value => {
                match self
                    .scope()
                    .borrow()
                    .get_item(&identifier.name)
                    .map_err(|error| Error::Scope(location, error))?
                {
                    ScopeItem::Variable(variable) => {
                        let size = variable.r#type.size();
                        let value = Value::new(variable.r#type);
                        self.add_instruction_load_push(variable.address, size);
                        Ok(Element::Value(value))
                    }
                    ScopeItem::Constant(constant) => {
                        self.instructions.push(constant.clone().into());
                        Ok(Element::Constant(constant))
                    }
                    ScopeItem::Static(r#static) => {
                        let r#type = r#static.data.r#type();
                        let size = r#type.size();
                        self.add_instruction_load_push(r#static.address, size);
                        Ok(Element::Constant(r#static.data))
                    }
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(identifier.name)),
                }
            }
            ResolutionMode::Place => {
                match self
                    .scope()
                    .borrow()
                    .get_item(&identifier.name)
                    .map_err(|error| Error::Scope(location, error))?
                {
                    ScopeItem::Variable(_variable) => Ok(Element::Place(Place::new(
                        location,
                        identifier.name.clone(),
                    ))),
                    ScopeItem::Constant(_constant) => Ok(Element::Place(Place::new(
                        location,
                        identifier.name.clone(),
                    ))),
                    ScopeItem::Static(_static) => Ok(Element::Place(Place::new(
                        location,
                        identifier.name.clone(),
                    ))),
                    ScopeItem::Type(r#type) => Ok(Element::Type(r#type)),
                    ScopeItem::Module(_) => Ok(Element::Module(identifier.name.to_owned())),
                }
            }
            ResolutionMode::Member => Ok(Element::MemberString(identifier.name)),
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
            self.expression(*expression, ResolutionMode::Value)
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
        let condition_result = self.expression(*conditional.condition, ResolutionMode::Value)?;
        match self.element_type(&condition_result)? {
            Type::Boolean => {}
            r#type => {
                return Err(Error::ConditionalExpectedBooleanCondition(
                    condition_location,
                    r#type,
                ))
            }
        }

        self.instructions.push(Instruction::If(zinc_bytecode::If));

        self.push_scope();
        let main_result = self.block_expression(conditional.main_block)?;
        let main_type = self.element_type(&main_result)?;
        self.pop_scope();

        let else_type = if let Some(else_block) = conditional.else_block {
            self.instructions
                .push(Instruction::Else(zinc_bytecode::Else));

            self.push_scope();
            let else_result = self.block_expression(else_block)?;
            let else_type = self.element_type(&else_result)?;
            self.pop_scope();

            else_type
        } else {
            Type::Unit
        };

        self.instructions
            .push(Instruction::EndIf(zinc_bytecode::EndIf));

        // check if the two branches return equals types
        if main_type != else_type {
            return Err(Error::ConditionalBranchTypesMismatch(
                location, main_type, else_type,
            ));
        }

        Ok(main_result)
    }

    fn match_expression(&mut self, r#match: MatchExpression) -> Result<Element, Error> {
        let location = r#match.location;

        let scrutinee_result = self.expression(r#match.scrutinee, ResolutionMode::Value)?;
        let scrutinee_type = self.element_type(&scrutinee_result)?;
        let scrutinee_size = scrutinee_type.size();
        let scrutinee_address = self.allocate_stack_space(scrutinee_size);
        self.add_instruction_pop_store(scrutinee_address, scrutinee_size);

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
                            pattern_type,
                            scrutinee_type,
                        ));
                    }

                    if index > 0 {
                        self.instructions
                            .push(Instruction::Else(zinc_bytecode::Else));
                        endifs += 1;
                    }

                    self.add_instruction_load_push(scrutinee_address, scrutinee_size);
                    self.instructions.push(constant.into());
                    self.instructions.push(Instruction::Eq(zinc_bytecode::Eq));
                    self.instructions.push(Instruction::If(zinc_bytecode::If));

                    self.expression(expression, ResolutionMode::Value)?
                }
                PatternVariant::IntegerLiteral(integer) => {
                    let constant = IntegerConstant::try_from(&integer).map_err(|error| {
                        Error::TypeInferenceMatchPattern(integer.location, error)
                    })?;
                    let pattern_type = constant.r#type();
                    if pattern_type != scrutinee_type {
                        return Err(Error::MatchBranchPatternInvalidType(
                            pattern_location,
                            pattern_type,
                            scrutinee_type,
                        ));
                    }

                    if index > 0 {
                        self.instructions
                            .push(Instruction::Else(zinc_bytecode::Else));
                        endifs += 1;
                    }

                    self.add_instruction_load_push(scrutinee_address, scrutinee_size);
                    self.instructions.push(constant.into());
                    self.instructions.push(Instruction::Eq(zinc_bytecode::Eq));
                    self.instructions.push(Instruction::If(zinc_bytecode::If));

                    self.expression(expression, ResolutionMode::Value)?
                }
                PatternVariant::Binding(identifier) => {
                    let location = identifier.location;
                    is_exhausted = true;

                    if index > 0 {
                        self.instructions
                            .push(Instruction::Else(zinc_bytecode::Else));
                    }
                    self.push_scope();
                    self.scope()
                        .borrow_mut()
                        .declare_variable(
                            identifier.name,
                            VariableItem::new(scrutinee_type.clone(), false, scrutinee_address),
                        )
                        .map_err(|error| Error::Scope(location, error))?;
                    let result = self.expression(expression, ResolutionMode::Value)?;
                    self.pop_scope();

                    if index > 0 {
                        self.instructions
                            .push(Instruction::EndIf(zinc_bytecode::EndIf));
                    }

                    result
                }
                PatternVariant::Ignoring => {
                    is_exhausted = true;

                    if index > 0 {
                        self.instructions
                            .push(Instruction::Else(zinc_bytecode::Else));
                    }

                    let result = self.expression(expression, ResolutionMode::Value)?;

                    if index > 0 {
                        self.instructions
                            .push(Instruction::EndIf(zinc_bytecode::EndIf));
                    }

                    result
                }
            };

            let result_type = self.element_type(&result)?;
            if result_type != scrutinee_type {
                return Err(Error::MatchBranchExpressionInvalidType(
                    expression_location,
                    result_type,
                    scrutinee_type,
                ));
            }

            branch_results.push(result);
        }

        for _ in 0..endifs {
            self.instructions
                .push(Instruction::EndIf(zinc_bytecode::EndIf));
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
            let element = self.expression(expression, ResolutionMode::Value)?;
            result
                .push(self.element_type(&element)?)
                .map_err(|error| Error::LiteralArray(location, error))?;
        }

        Ok(Element::Value(Value::Array(result)))
    }

    fn tuple_expression(&mut self, tuple: TupleExpression) -> Result<Element, Error> {
        let mut result = Tuple::default();
        for expression in tuple.elements.into_iter() {
            let element = self.expression(expression, ResolutionMode::Value)?;
            result.push(self.element_type(&element)?);
        }

        Ok(Element::Value(Value::Tuple(result)))
    }

    fn structure_expression(&mut self, structure: StructureExpression) -> Result<Element, Error> {
        let path_location = structure.path.location;
        let identifier = match self.expression(structure.path, ResolutionMode::Place)? {
            Element::Type(Type::Structure { identifier, .. }) => identifier,
            element => return Err(Error::ExpectedType(path_location, element)),
        };

        let mut result = Structure::new(identifier, Vec::with_capacity(structure.fields.len()));
        for (identifier, expression) in structure.fields.into_iter() {
            let location = identifier.location;
            let element = self.expression(expression, ResolutionMode::Value)?;
            result
                .push(identifier.name, self.element_type(&element)?)
                .map_err(|error| Error::LiteralStructure(location, error))?;
        }

        Ok(Element::Value(Value::Structure(result)))
    }

    fn list_expression(&mut self, list: Vec<Expression>) -> Result<Element, Error> {
        let mut elements = Vec::with_capacity(list.len());
        for expression in list.into_iter() {
            let element = self.expression(expression, ResolutionMode::Value)?;
            let argument_address = self.allocate_stack_space(self.element_type(&element)?.size());
            elements.push(element);
            self.instructions
                .push(Instruction::PopStore(zinc_bytecode::PopStore::new(
                    argument_address,
                )));
        }
        Ok(Element::ArgumentList(elements))
    }

    fn evaluate_operand(&mut self, resolution_mode: ResolutionMode) -> Result<Element, Error> {
        match self.pop_operand() {
            StackElement::NotEvaluated(operand) => match operand {
                ExpressionOperand::Unit => Ok(Element::Value(Value::Unit)),
                ExpressionOperand::LiteralBoolean(literal) => self.boolean_literal(literal),
                ExpressionOperand::LiteralInteger(literal) => self.integer_literal(literal),
                ExpressionOperand::LiteralString(literal) => self.string_literal(literal),
                ExpressionOperand::MemberInteger(integer) => self.member_integer(integer),
                ExpressionOperand::MemberString(string) => self.member_string(string),
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
                (ResolutionMode::Value, Element::Place(place)) => {
                    let location = place.location;
                    let variable = self
                        .scope()
                        .borrow()
                        .get_variable(&place)
                        .map_err(|error| Error::Scope(location, error))?;
                    let size = variable.r#type.size();
                    self.add_instruction_load_push(variable.address, size);
                    let value = Value::new(variable.r#type);
                    Ok(Element::Value(value))
                }
                (_resolution_mode, element) => Ok(element),
            },
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
                match self.expression(path, ResolutionMode::Place)? {
                    Element::Type(r#type) => r#type,
                    element => return Err(Error::ExpectedType(location, element)),
                }
            }
        })
    }

    fn element_type(&self, element: &Element) -> Result<Type, Error> {
        Ok(match element {
            Element::Place(place) => {
                let location = place.location;
                self.scope()
                    .borrow()
                    .get_variable(&place)
                    .map_err(|error| Error::Scope(location, error))?
                    .r#type
            }
            Element::Value(value) => value.r#type(),
            Element::Constant(constant) => constant.r#type(),
            Element::Type(r#type) => r#type.to_owned(),
            _ => panic!(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS),
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

    fn evaluate_unary_operand(
        &mut self,
        resolution_mode: ResolutionMode,
    ) -> Result<Element, Error> {
        self.evaluate_operand(resolution_mode)
    }

    fn evaluate_binary_operands(
        &mut self,
        resolution_mode_1: ResolutionMode,
        resolution_mode_2: ResolutionMode,
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

    fn allocate_stack_space(&mut self, size: usize) -> usize {
        self.stack_position += size;
        self.stack_position - size
    }

    fn add_instruction_load_push(&mut self, address: usize, size: usize) {
        match size {
            0 => {}
            1 => self
                .instructions
                .push(Instruction::LoadPush(zinc_bytecode::LoadPush::new(address))),
            size => self.instructions.push(Instruction::LoadPushArray(
                zinc_bytecode::LoadPushArray::new(address, size),
            )),
        }
    }

    fn add_instruction_pop_store(&mut self, address: usize, size: usize) {
        match size {
            0 => {}
            1 => self
                .instructions
                .push(Instruction::PopStore(zinc_bytecode::PopStore::new(address))),
            size => self.instructions.push(Instruction::PopStoreArray(
                zinc_bytecode::PopStoreArray::new(address, size),
            )),
        }
    }
}
