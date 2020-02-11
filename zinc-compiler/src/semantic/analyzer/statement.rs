//!
//! The statement semantic analyzer.
//!

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;

use zinc_bytecode::Instruction;

use crate::semantic::analyzer::error::Error;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::translation_hint::TranslationHint;
use crate::semantic::bytecode::Bytecode;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::function::user::Function as UserDefinedFunctionType;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::r#type::UNIQUE_ID;
use crate::semantic::element::Element;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::BindingPatternVariant;
use crate::syntax::ConstStatement;
use crate::syntax::EnumStatement;
use crate::syntax::FnStatement;
use crate::syntax::FunctionLocalStatement;
use crate::syntax::ImplStatement;
use crate::syntax::ImplementationLocalStatement;
use crate::syntax::LetStatement;
use crate::syntax::LoopStatement;
use crate::syntax::ModStatement;
use crate::syntax::ModuleLocalStatement;
use crate::syntax::StaticStatement;
use crate::syntax::StructStatement;
use crate::syntax::TypeStatement;
use crate::syntax::UseStatement;

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

    pub fn module_local_statement(&mut self, statement: ModuleLocalStatement) -> Result<(), Error> {
        match statement {
            ModuleLocalStatement::Const(statement) => self.const_statement(statement)?,
            ModuleLocalStatement::Static(statement) => self.static_statement(statement)?,
            ModuleLocalStatement::Type(statement) => self.type_statement(statement)?,
            ModuleLocalStatement::Struct(statement) => self.struct_statement(statement)?,
            ModuleLocalStatement::Enum(statement) => self.enum_statement(statement)?,
            ModuleLocalStatement::Fn(statement) => self.fn_statement(statement)?,
            ModuleLocalStatement::Mod(statement) => self.mod_statement(statement)?,
            ModuleLocalStatement::Use(statement) => self.use_statement(statement)?,
            ModuleLocalStatement::Impl(statement) => self.impl_statement(statement)?,
        }

        Ok(())
    }

    pub fn function_local_statement(
        &mut self,
        statement: FunctionLocalStatement,
    ) -> Result<(), Error> {
        match statement {
            FunctionLocalStatement::Let(statement) => self.let_statement(statement)?,
            FunctionLocalStatement::Const(statement) => self.const_statement(statement)?,
            FunctionLocalStatement::Loop(statement) => self.loop_statement(statement)?,
            FunctionLocalStatement::Expression(expression) => {
                ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                    .expression(expression, TranslationHint::ValueExpression)?;
            }
        }

        Ok(())
    }

    pub fn implementation_local_statement(
        &mut self,
        statement: ImplementationLocalStatement,
    ) -> Result<(), Error> {
        match statement {
            ImplementationLocalStatement::Const(statement) => self.const_statement(statement)?,
            ImplementationLocalStatement::Fn(statement) => self.fn_statement(statement)?,
        }

        Ok(())
    }

    fn const_statement(&mut self, statement: ConstStatement) -> Result<(), Error> {
        let location = statement.location;
        let type_location = statement.r#type.location;
        let expression_location = statement.expression.location;

        // compile the expression being assigned
        let mut rvalue = ExpressionAnalyzer::new_without_bytecode(self.scope())
            .expression(statement.expression, TranslationHint::ValueExpression)?;

        let const_type = Type::from_type_variant(&statement.r#type.variant, self.scope())?;
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
        let expression_location = statement.expression.location;

        // compile the expression being assigned
        let mut rvalue = ExpressionAnalyzer::new_without_bytecode(self.scope())
            .expression(statement.expression, TranslationHint::ValueExpression)?;

        let const_type = Type::from_type_variant(&statement.r#type.variant, self.scope())?;
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

    fn type_statement(&mut self, statement: TypeStatement) -> Result<(), Error> {
        let location = statement.location;

        let r#type = Type::from_type_variant(&statement.r#type.variant, self.scope())?;

        self.scope()
            .borrow_mut()
            .declare_type(statement.identifier.name, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn struct_statement(&mut self, statement: StructStatement) -> Result<(), Error> {
        let location = statement.location;

        let mut fields = Vec::with_capacity(statement.fields.len());
        for field in statement.fields.into_iter() {
            fields.push((
                field.identifier.name,
                Type::from_type_variant(&field.r#type.variant, self.scope())?,
            ));
        }

        unsafe {
            UNIQUE_ID += 1;
        }
        let r#type = Type::structure(
            statement.identifier.name.clone(),
            unsafe { UNIQUE_ID },
            fields,
            Some(self.scope()),
        );

        self.scope()
            .borrow_mut()
            .declare_type(statement.identifier.name, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn enum_statement(&mut self, statement: EnumStatement) -> Result<(), Error> {
        let location = statement.location;

        unsafe {
            UNIQUE_ID += 1;
        }
        let r#type = Type::enumeration(
            statement.identifier.clone(),
            unsafe { UNIQUE_ID },
            statement.variants,
            Some(self.scope()),
        )?;

        self.scope()
            .borrow_mut()
            .declare_type(statement.identifier.name, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn fn_statement(&mut self, statement: FnStatement) -> Result<(), Error> {
        let location = statement.location;

        let identifier = statement.identifier.name;

        let mut argument_bindings = Vec::with_capacity(statement.argument_bindings.len());
        for argument_binding in statement.argument_bindings.iter() {
            let identifier = match argument_binding.variant {
                BindingPatternVariant::Binding(ref identifier) => identifier,
                BindingPatternVariant::MutableBinding(ref identifier) => identifier,
                BindingPatternVariant::Wildcard => continue,
            };
            argument_bindings.push((
                identifier.name.clone(),
                Type::from_type_variant(&argument_binding.r#type.variant, self.scope())?,
            ));
        }
        let return_type = Type::from_type_variant(&statement.return_type.variant, self.scope())?;

        let unique_id = unsafe {
            UNIQUE_ID += 1;
            UNIQUE_ID
        };
        let function_type = UserDefinedFunctionType::new(
            identifier.clone(),
            unique_id,
            argument_bindings,
            return_type,
        );
        let r#type = Type::Function(FunctionType::UserDefined(function_type));

        self.scope()
            .borrow_mut()
            .declare_type(identifier.clone(), r#type)
            .map_err(|error| Error::Scope(location, error))?;

        // record the function address in the bytecode
        self.bytecode
            .borrow_mut()
            .start_new_function(&identifier, unique_id);

        // start a new scope and declare the function arguments there
        self.push_scope();
        for argument_binding in statement.argument_bindings.into_iter() {
            let (identifier, is_mutable) = match argument_binding.variant {
                BindingPatternVariant::Binding(identifier) => (identifier, false),
                BindingPatternVariant::MutableBinding(identifier) => (identifier, true),
                BindingPatternVariant::Wildcard => continue,
            };
            let r#type = Type::from_type_variant(&argument_binding.r#type.variant, self.scope())?;
            let address = self
                .bytecode
                .borrow_mut()
                .allocate_data_stack_space(r#type.size());
            self.scope()
                .borrow_mut()
                .declare_variable(
                    identifier.name,
                    ScopeVariableItem::new(r#type, is_mutable, address),
                )
                .map_err(|error| Error::Scope(location, error))?;
        }

        // compile the function block
        let result = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
            .block_expression(statement.body)?;
        self.pop_scope();

        // check the function return type to match the block result
        let result_type = Type::from_element(&result, self.scope())?;
        let expected_type = Type::from_type_variant(&statement.return_type.variant, self.scope())?;
        if expected_type != result_type {
            return Err(Error::FunctionReturnTypeMismatch(
                statement.return_type.location,
                identifier,
                expected_type.to_string(),
                result_type.to_string(),
            ));
        }

        self.bytecode.borrow_mut().push_instruction(
            Instruction::Return(zinc_bytecode::Return::new(expected_type.size())),
            statement.return_type.location,
        );

        Ok(())
    }

    fn mod_statement(&mut self, statement: ModStatement) -> Result<(), Error> {
        let identifier_location = statement.identifier.location;
        let module = match self.dependencies.remove(statement.identifier.name.as_str()) {
            Some(module) => module,
            None => {
                return Err(Error::ModuleNotFound(
                    identifier_location,
                    statement.identifier.name,
                ))
            }
        };
        self.scope()
            .borrow_mut()
            .declare_module(statement.identifier.name, module)
            .map_err(|error| Error::Scope(identifier_location, error))?;

        Ok(())
    }

    fn use_statement(&mut self, statement: UseStatement) -> Result<(), Error> {
        let path_location = statement.path.location;

        let path = match ExpressionAnalyzer::new_without_bytecode(self.scope())
            .expression(statement.path, TranslationHint::PathExpression)?
        {
            Element::Path(path) => path,
            element => return Err(Error::UseExpectedPath(path_location, element.to_string())),
        };
        let item = Scope::resolve_path(self.scope(), &path)?;
        let last_member_string = path
            .elements
            .last()
            .expect(crate::semantic::PANIC_THERE_MUST_ALWAYS_BE_THE_LAST_PATH_ELEMENT);
        self.scope()
            .borrow_mut()
            .declare_item(last_member_string.name.to_owned(), item)
            .map_err(|error| Error::Scope(last_member_string.location, error))?;

        Ok(())
    }

    fn impl_statement(&mut self, statement: ImplStatement) -> Result<(), Error> {
        let identifier_location = statement.identifier.location;

        let structure_scope =
            match Scope::resolve_item(self.scope(), statement.identifier.name.as_str())
                .map_err(|error| Error::Scope(identifier_location, error))?
            {
                ScopeItem::Type(Type::Structure(structure)) => structure.scope,
                ScopeItem::Type(Type::Enumeration { scope, .. }) => scope,
                item => {
                    return Err(Error::ImplStatementExpectedStructureOrEnumeration(
                        identifier_location,
                        item.to_string(),
                    ))
                }
            };

        self.scope_stack.push(structure_scope);
        for statement in statement.statements.into_iter() {
            self.implementation_local_statement(statement)?;
        }
        self.pop_scope();

        Ok(())
    }

    fn let_statement(&mut self, statement: LetStatement) -> Result<(), Error> {
        let location = statement.location;

        // compile the expression being assigned
        let mut rvalue = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
            .expression(statement.expression, TranslationHint::ValueExpression)?;

        let r#type = if let Some(r#type) = statement.r#type {
            let type_location = r#type.location;
            let let_type = Type::from_type_variant(&r#type.variant, self.scope())?;

            if let Some((is_signed, bitlength)) = rvalue
                .cast(&Element::Type(let_type.clone()))
                .map_err(|error| Error::Element(type_location, error))?
            {
                self.bytecode.borrow_mut().push_instruction(
                    Instruction::Cast(zinc_bytecode::Cast::new(is_signed, bitlength)),
                    type_location,
                );
            }
            let_type
        } else {
            Type::from_element(&rvalue, self.scope())?
        };

        let size = r#type.size();
        let address = self.bytecode.borrow_mut().allocate_data_stack_space(size);
        self.bytecode
            .borrow_mut()
            .push_instruction_store(address, size, None, false, location);
        self.scope()
            .borrow_mut()
            .declare_variable(
                statement.identifier.name,
                ScopeVariableItem::new(r#type, statement.is_mutable, address),
            )
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn loop_statement(&mut self, statement: LoopStatement) -> Result<(), Error> {
        let location = statement.location;
        let bounds_expression_location = statement.bounds_expression.location;

        let (range_start, range_end, bitlength, is_signed, is_inclusive) =
            match ExpressionAnalyzer::new_without_bytecode(self.scope()).expression(
                statement.bounds_expression,
                TranslationHint::ValueExpression,
            )? {
                Element::Constant(Constant::RangeInclusive(range)) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    true,
                ),
                Element::Constant(Constant::Range(range)) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    false,
                ),
                element => {
                    return Err(Error::LoopBoundsExpectedConstantRangeExpression(
                        bounds_expression_location,
                        element.to_string(),
                    ))
                }
            };

        // calculate the iterations number and if the loop is reverse
        let iterations_count = cmp::max(&range_start, &range_end)
            - cmp::min(&range_start, &range_end)
            + if is_inclusive {
                BigInt::one()
            } else {
                BigInt::zero()
            };
        let is_reverse = range_start > range_end;

        // create the index value and get its address
        let index = IntegerConstant::new(range_start, is_signed, bitlength);
        let index_type = index.r#type();
        let index_size = index_type.size();
        let index_address = self
            .bytecode
            .borrow_mut()
            .allocate_data_stack_space(index_size);
        self.bytecode
            .borrow_mut()
            .push_instruction(index.to_instruction(), bounds_expression_location);
        self.bytecode.borrow_mut().push_instruction_store(
            index_address,
            index_size,
            None,
            false,
            bounds_expression_location,
        );

        // create the while allowed condition
        let while_allowed_address = match statement.while_condition {
            Some(ref condition) => {
                let while_allowed = Constant::Boolean(true);
                let while_allowed_address = self
                    .bytecode
                    .borrow_mut()
                    .allocate_data_stack_space(while_allowed.r#type().size());
                self.bytecode
                    .borrow_mut()
                    .push_instruction(while_allowed.to_instruction(), condition.location);
                self.bytecode.borrow_mut().push_instruction(
                    Instruction::Store(zinc_bytecode::Store::new(while_allowed_address)),
                    condition.location,
                );
                Some(while_allowed_address)
            }
            None => None,
        };

        let iterations_count = iterations_count.to_usize().ok_or_else(|| {
            Error::InferenceConstant(
                location,
                IntegerConstantError::LiteralTooLargeForIndex(
                    iterations_count.to_string(),
                    crate::BITLENGTH_BYTE,
                ),
            )
        })?;
        self.bytecode.borrow_mut().push_instruction(
            Instruction::LoopBegin(zinc_bytecode::LoopBegin::new(iterations_count)),
            bounds_expression_location,
        );

        // declare the index variable
        self.push_scope();
        self.scope()
            .borrow_mut()
            .declare_variable(
                statement.index_identifier.name,
                ScopeVariableItem::new(Type::scalar(is_signed, bitlength), false, index_address),
            )
            .map_err(|error| Error::Scope(location, error))?;

        // check the while condition, set the allowed variable, and execute the loop body
        if let (Some(expression), Some(while_allowed_address)) =
            (statement.while_condition, while_allowed_address)
        {
            let location = expression.location;
            let while_result = ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                .expression(expression, TranslationHint::ValueExpression)?;

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
                .push_instruction(Instruction::Not(zinc_bytecode::Not), location);
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), location);
            self.bytecode
                .borrow_mut()
                .push_instruction(Constant::Boolean(false).to_instruction(), location);
            self.bytecode.borrow_mut().push_instruction_store(
                while_allowed_address,
                Type::boolean().size(),
                None,
                false,
                location,
            );
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);

            self.bytecode.borrow_mut().push_instruction_load(
                while_allowed_address,
                Type::boolean().size(),
                None,
                false,
                location,
            );
            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::If(zinc_bytecode::If), location);

            ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                .block_expression(statement.block)?;

            self.bytecode
                .borrow_mut()
                .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
        } else {
            ExpressionAnalyzer::new(self.scope(), self.bytecode.clone())
                .block_expression(statement.block)?;
        }

        // increment or decrement the loop counter
        self.bytecode.borrow_mut().push_instruction(
            Instruction::Load(zinc_bytecode::Load::new(index_address)),
            location,
        );
        self.bytecode.borrow_mut().push_instruction(
            IntegerConstant::new_one(is_signed, bitlength).to_instruction(),
            location,
        );
        self.bytecode.borrow_mut().push_instruction(
            if is_reverse {
                Instruction::Sub(zinc_bytecode::Sub)
            } else {
                Instruction::Add(zinc_bytecode::Add)
            },
            location,
        );
        self.bytecode.borrow_mut().push_instruction(
            Instruction::Store(zinc_bytecode::Store::new(index_address)),
            location,
        );
        self.bytecode
            .borrow_mut()
            .push_instruction(Instruction::LoopEnd(zinc_bytecode::LoopEnd), location);

        self.pop_scope();

        Ok(())
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
