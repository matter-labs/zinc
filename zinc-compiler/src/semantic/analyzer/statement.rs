//!
//! The statement semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::semantic::Bytecode;
use crate::semantic::Constant;
use crate::semantic::Element;
use crate::semantic::Error;
use crate::semantic::ExpressionAnalyzer;
use crate::semantic::IntegerConstant;
use crate::semantic::ResolutionHint;
use crate::semantic::Scope;
use crate::semantic::ScopeStaticItem;
use crate::semantic::ScopeVariableItem;
use crate::semantic::Type;
use crate::syntax::ConstStatement;
use crate::syntax::InnerStatement;
use crate::syntax::OuterStatement;
use crate::syntax::StaticStatement;

pub struct Analyzer {
    scope_stack: Vec<Rc<RefCell<Scope>>>,
    bytecode: Rc<RefCell<Bytecode>>,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
}

impl Analyzer {
    const STACK_SCOPE_INITIAL_CAPACITY: usize = 16;

    pub fn new(
        scope: Rc<RefCell<Scope>>,
        bytecode: Rc<RefCell<Bytecode>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Self {
        Self {
            scope_stack: {
                let mut scope_stack = Vec::with_capacity(Self::STACK_SCOPE_INITIAL_CAPACITY);
                scope_stack.push(scope);
                scope_stack
            },
            bytecode,
            dependencies,
        }
    }

    pub fn outer_statement(&mut self, statement: OuterStatement) -> Result<(), Error> {
        match statement {
            OuterStatement::Const(statement) => self.const_statement(statement)?,
            OuterStatement::Static(statement) => self.static_statement(statement)?,
            OuterStatement::Type(statement) => {
                let location = statement.location;

                let r#type = Type::from_type_variant(statement.r#type.variant, self.scope())?;

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
                        Type::from_type_variant(field.r#type.variant, self.scope())?,
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

                let r#type =
                    Type::new_enumeration(statement.identifier.clone(), statement.variants)?;

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
                        Type::from_type_variant(argument.r#type.variant.clone(), self.scope())?,
                    ));
                }
                let return_type =
                    Type::from_type_variant(statement.return_type.variant.clone(), self.scope())?;
                let r#type = Type::new_function(identifier.clone(), arguments, return_type);

                self.scope()
                    .borrow_mut()
                    .declare_type(identifier.clone(), r#type)
                    .map_err(|error| Error::Scope(location, error))?;

                // record the function address in the bytecode
                self.bytecode.borrow_mut().start_new_function(&identifier);

                // start a new scope and declare the function arguments there
                self.push_scope();
                for argument in statement.arguments.into_iter() {
                    let r#type = Type::from_type_variant(argument.r#type.variant, self.scope())?;
                    let address = self
                        .bytecode
                        .borrow_mut()
                        .allocate_stack_space(r#type.size());
                    self.scope()
                        .borrow_mut()
                        .declare_variable(
                            argument.identifier.name,
                            ScopeVariableItem::new(r#type, false, address),
                        )
                        .map_err(|error| Error::Scope(location, error))?;
                }

                // compile the function block
                let result = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                    .block_expression(statement.body)?;
                self.pop_scope();

                // check the function return type to match the block result
                let result_type = Type::from_element(&result, self.scope())?;
                let expected_type =
                    Type::from_type_variant(statement.return_type.variant, self.scope())?;
                if expected_type != result_type {
                    return Err(Error::FunctionReturnTypeMismatch(
                        statement.return_type.location,
                        identifier,
                        expected_type.to_string(),
                        result_type.to_string(),
                    ));
                }

                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::Return(zinc_bytecode::Return::new(
                        expected_type.size(),
                    )));
            }
            OuterStatement::Mod(statement) => {
                let location = statement.location;
                let module = match self.dependencies.remove(statement.identifier.name.as_str()) {
                    Some(module) => module,
                    None => return Err(Error::ModuleNotFound(location, statement.identifier.name)),
                };
                self.scope()
                    .borrow_mut()
                    .declare_module(statement.identifier.name, module)
                    .map_err(|error| Error::Scope(location, error))?;
            }
            OuterStatement::Use(_statement) => {}
        }

        Ok(())
    }

    pub fn inner_statement(&mut self, statement: InnerStatement) -> Result<(), Error> {
        match statement {
            InnerStatement::Let(statement) => {
                let location = statement.location;

                // compile the expression being assigned
                let mut rvalue = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                    .expression(statement.expression, ResolutionHint::ValueExpression)?;

                let r#type = if let Some(r#type) = statement.r#type {
                    let type_location = r#type.location;
                    let let_type = Type::from_type_variant(r#type.variant, self.scope())?;

                    if let Some((is_signed, bitlength)) = rvalue
                        .cast(&Element::Type(let_type.clone()))
                        .map_err(|error| Error::Element(type_location, error))?
                    {
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::Cast(zinc_bytecode::Cast::new(
                                is_signed,
                                bitlength as u8,
                            )));
                    }
                    let_type
                } else {
                    Type::from_element(&rvalue, self.scope())?
                };

                let size = r#type.size();
                let address = self.bytecode.borrow_mut().allocate_stack_space(size);
                self.bytecode
                    .borrow_mut()
                    .push_instruction_pop_store(address, size);
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
                .map_err(|error| Error::InferenceLoopBounds(location, error))?;

                // calculate the iterations number and if the loop is reverse
                let range_start: usize = statement.range_start.into();
                let range_end: usize = statement.range_end.into();
                let iterations_count = cmp::max(range_start, range_end)
                    - cmp::min(range_start, range_end)
                    + if statement.is_range_inclusive { 1 } else { 0 };
                let is_reverse = range_start > range_end;

                // create the index value and get its address
                let index = IntegerConstant::new_range_bound(range_start, range_bitlength);
                let index_type = index.r#type();
                let index_size = index_type.size();
                let index_address = self.bytecode.borrow_mut().allocate_stack_space(index_size);
                self.bytecode.borrow_mut().push_instruction(index.into());
                self.bytecode
                    .borrow_mut()
                    .push_instruction_pop_store(index_address, index_size);

                // create the while allowed condition
                let while_allowed_address = match statement.while_condition {
                    Some(_) => {
                        let while_allowed = Constant::Boolean(true);
                        let while_allowed_address = self
                            .bytecode
                            .borrow_mut()
                            .allocate_stack_space(while_allowed.r#type().size());
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(while_allowed.into());
                        self.bytecode
                            .borrow_mut()
                            .push_instruction(Instruction::PopStore(zinc_bytecode::PopStore::new(
                                while_allowed_address,
                            )));
                        Some(while_allowed_address)
                    }
                    None => None,
                };

                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(
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
                    let while_result = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                        .expression(expression, ResolutionHint::ValueExpression)?;

                    match Type::from_element(&while_result, self.scope())? {
                        Type::Boolean => {}
                        r#type => {
                            return Err(Error::LoopWhileExpectedBooleanCondition(
                                location,
                                r#type.to_string(),
                            ))
                        }
                    }

                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::Not(zinc_bytecode::Not));
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If));
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Constant::Boolean(false).into());
                    self.bytecode.borrow_mut().push_instruction_pop_store(
                        while_allowed_address,
                        Type::new_boolean().size(),
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));

                    self.bytecode.borrow_mut().push_instruction_load_push(
                        while_allowed_address,
                        Type::new_boolean().size(),
                    );
                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::If(zinc_bytecode::If));

                    ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                        .block_expression(statement.block)?;

                    self.bytecode
                        .borrow_mut()
                        .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf));
                } else {
                    ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                        .block_expression(statement.block)?;
                }

                // increment the loop counter
                self.bytecode
                    .borrow_mut()
                    .push_instruction(IntegerConstant::new_one(range_bitlength).into());
                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::LoadPush(zinc_bytecode::LoadPush::new(
                        index_address,
                    )));
                self.bytecode.borrow_mut().push_instruction(if is_reverse {
                    Instruction::Sub(zinc_bytecode::Sub)
                } else {
                    Instruction::Add(zinc_bytecode::Add)
                });
                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::PopStore(zinc_bytecode::PopStore::new(
                        index_address,
                    )));
                self.bytecode
                    .borrow_mut()
                    .push_instruction(Instruction::LoopEnd(zinc_bytecode::LoopEnd));

                self.pop_scope();
            }
            InnerStatement::Expression(expression) => {
                ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                    .expression(expression, ResolutionHint::ValueExpression)?;
            }
        }

        Ok(())
    }

    fn const_statement(&mut self, statement: ConstStatement) -> Result<(), Error> {
        let location = statement.location;
        let type_location = statement.r#type.location;
        let expression_location = statement.expression.location;

        // compile the expression being assigned
        let mut rvalue = ExpressionAnalyzer::new_without_bytecode(self.scope())
            .expression(statement.expression, ResolutionHint::ValueExpression)?;

        let const_type = Type::from_type_variant(statement.r#type.variant, self.scope())?;
        rvalue
            .cast(&Element::Type(const_type))
            .map_err(|error| Error::Element(type_location, error))?;
        let constant = match rvalue {
            Element::Constant(constant) => constant,
            element => {
                return Err(Error::ConstantExpressionHasNonConstantElement(
                    expression_location,
                    element.to_string(),
                ))
            }
        };

        self.scope()
            .borrow_mut()
            .declare_constant(statement.identifier.name, constant)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn static_statement(&mut self, statement: StaticStatement) -> Result<(), Error> {
        let location = statement.location;
        let type_location = statement.r#type.location;

        // compile the expression being assigned
        let mut rvalue = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
            .expression(statement.expression, ResolutionHint::ValueExpression)?;

        let const_type = Type::from_type_variant(statement.r#type.variant, self.scope())?;
        rvalue
            .cast(&Element::Type(const_type))
            .map_err(|error| Error::Element(type_location, error))?;
        let constant = match rvalue {
            Element::Constant(constant) => constant,
            element => {
                return Err(Error::ConstantExpressionHasNonConstantElement(
                    location,
                    element.to_string(),
                ))
            }
        };

        let size = constant.r#type().size();
        let address = self.bytecode.borrow_mut().allocate_stack_space(size);
        self.bytecode
            .borrow_mut()
            .push_instruction_pop_store(address, size);
        self.scope()
            .borrow_mut()
            .declare_static(
                statement.identifier.name,
                ScopeStaticItem::new(constant, address),
            )
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
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
}
