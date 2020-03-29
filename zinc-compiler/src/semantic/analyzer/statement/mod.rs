//!
//! The statement semantic analyzer.
//!

mod tests;

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

use num_traits::ToPrimitive;

use crate::generator::statement::declaration::Statement as GeneratorDeclarationStatement;
use crate::generator::statement::function::Statement as GeneratorFunctionStatement;
use crate::generator::statement::loop_for::Statement as GeneratorForLoopStatement;
use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::user::Function as UserDefinedFunctionType;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::r#type::TYPE_INDEX;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Variant as ScopeItem;
use crate::semantic::scope::stack::Stack as ScopeStack;
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
use crate::syntax::StructStatement;
use crate::syntax::TypeStatement;
use crate::syntax::UseStatement;

pub struct Analyzer {
    scope_stack: ScopeStack,
    dependencies: HashMap<String, Rc<RefCell<Scope>>>,
}

impl Analyzer {
    pub fn new(
        scope: Rc<RefCell<Scope>>,
        dependencies: HashMap<String, Rc<RefCell<Scope>>>,
    ) -> Self {
        Self {
            scope_stack: ScopeStack::new(scope),
            dependencies,
        }
    }

    pub fn local_mod(
        &mut self,
        statement: ModuleLocalStatement,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            ModuleLocalStatement::Const(statement) => {
                self.r#const(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Type(statement) => {
                self.r#type(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Struct(statement) => {
                self.r#struct(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Enum(statement) => {
                self.r#enum(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Fn(statement) => {
                let intermediate = GeneratorStatement::Function(self.r#fn(statement)?);
                Ok(Some(intermediate))
            }
            ModuleLocalStatement::Mod(statement) => {
                self.r#mod(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Use(statement) => {
                self.r#use(statement)?;
                Ok(None)
            }
            ModuleLocalStatement::Impl(statement) => {
                let intermediate = GeneratorStatement::Implementation(self.r#impl(statement)?);
                Ok(Some(intermediate))
            }
            ModuleLocalStatement::Empty(_location) => Ok(None),
        }
    }

    pub fn local_fn(
        &mut self,
        statement: FunctionLocalStatement,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            FunctionLocalStatement::Let(statement) => {
                Ok(self.r#let(statement)?.map(GeneratorStatement::Declaration))
            }
            FunctionLocalStatement::Const(statement) => {
                self.r#const(statement)?;
                Ok(None)
            }
            FunctionLocalStatement::Loop(statement) => {
                Ok(Some(GeneratorStatement::Loop(self.r#for(statement)?)))
            }
            FunctionLocalStatement::Expression(expression) => {
                let (_result, expression) = ExpressionAnalyzer::new(self.scope_stack.top())
                    .analyze(expression, TranslationHint::ValueExpression)?;
                let intermediate = GeneratorStatement::Expression(expression);
                Ok(Some(intermediate))
            }
            FunctionLocalStatement::Empty(_location) => Ok(None),
        }
    }

    pub fn local_impl(
        &mut self,
        statement: ImplementationLocalStatement,
    ) -> Result<Option<GeneratorStatement>, Error> {
        match statement {
            ImplementationLocalStatement::Const(statement) => {
                self.r#const(statement)?;
                Ok(None)
            }
            ImplementationLocalStatement::Fn(statement) => {
                let intermediate = GeneratorStatement::Function(self.r#fn(statement)?);
                Ok(Some(intermediate))
            }
            ImplementationLocalStatement::Empty(_location) => Ok(None),
        }
    }

    fn r#fn(&mut self, statement: FnStatement) -> Result<GeneratorFunctionStatement, Error> {
        let location = statement.location;

        let mut arguments = Vec::with_capacity(statement.argument_bindings.len());
        for argument_binding in statement.argument_bindings.iter() {
            let identifier = match argument_binding.variant {
                BindingPatternVariant::Binding(ref identifier) => identifier,
                BindingPatternVariant::MutableBinding(ref identifier) => identifier,
                BindingPatternVariant::Wildcard => continue,
            };
            arguments.push((
                identifier.name.clone(),
                Type::from_type_variant(&argument_binding.r#type.variant, self.scope_stack.top())?,
            ));
        }
        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::from_type_variant(&r#type.variant, self.scope_stack.top())?,
            None => Type::unit(),
        };

        let unique_id = TYPE_INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        let function_type = UserDefinedFunctionType::new(
            statement.identifier.name.clone(),
            unique_id,
            arguments.clone(),
            expected_type.clone(),
        );
        let r#type = Type::Function(FunctionType::UserDefined(function_type));

        TYPE_INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());
        Scope::declare_type(self.scope_stack.top(), statement.identifier.clone(), r#type)
            .map_err(|error| Error::Scope(location, error))?;

        self.scope_stack.push();
        for argument_binding in statement.argument_bindings.into_iter() {
            let (identifier, is_mutable) = match argument_binding.variant {
                BindingPatternVariant::Binding(identifier) => (identifier, false),
                BindingPatternVariant::MutableBinding(identifier) => (identifier, true),
                BindingPatternVariant::Wildcard => continue,
            };
            let identifier_location = identifier.location;
            let r#type =
                Type::from_type_variant(&argument_binding.r#type.variant, self.scope_stack.top())?;

            Scope::declare_variable(
                self.scope_stack.top(),
                identifier,
                ScopeVariableItem::new(is_mutable, r#type),
            )
            .map_err(|error| Error::Scope(identifier_location, error))?;
        }

        let return_expression_location = match statement
            .body
            .expression
            .as_ref()
            .map(|expression| expression.location)
        {
            Some(location) => location,
            None => statement
                .body
                .statements
                .last()
                .map(|statement| statement.location())
                .unwrap_or(statement.location),
        };
        let (result, body) = BlockAnalyzer::analyze(self.scope_stack.top(), statement.body)?;
        self.scope_stack.pop();

        let result_type = Type::from_element(&result, self.scope_stack.top())?;
        if expected_type != result_type {
            return Err(Error::Function(
                return_expression_location,
                FunctionError::return_type(
                    statement.identifier.name.clone(),
                    expected_type.to_string(),
                    result_type.to_string(),
                    statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                ),
            ));
        }

        let is_main = statement.identifier.name.as_str()
            == crate::semantic::element::r#type::function::user::FUNCTION_MAIN_IDENTIFIER;

        Ok(GeneratorFunctionStatement::new(
            statement.identifier.name,
            arguments,
            body,
            expected_type,
            unique_id,
            is_main,
        ))
    }

    fn r#impl(&mut self, statement: ImplStatement) -> Result<Vec<GeneratorStatement>, Error> {
        let identifier_location = statement.identifier.location;

        let mut intermediate = Vec::new();

        let structure_scope =
            match Scope::resolve_item(self.scope_stack.top(), statement.identifier.name.as_str())
                .map_err(|error| Error::Scope(identifier_location, error))?
                .variant
            {
                ScopeItem::Type(Type::Structure(structure)) => structure.scope,
                ScopeItem::Type(Type::Enumeration(enumeration)) => enumeration.scope,
                item => {
                    return Err(Error::ImplStatementExpectedStructureOrEnumeration {
                        location: identifier_location,
                        found: item.to_string(),
                    });
                }
            };

        self.scope_stack.push_scope(structure_scope);
        for statement in statement.statements.into_iter() {
            if let Some(statement) = self.local_impl(statement)? {
                intermediate.push(statement);
            }
        }
        self.scope_stack.pop();

        Ok(intermediate)
    }

    fn r#let(
        &mut self,
        statement: LetStatement,
    ) -> Result<Option<GeneratorDeclarationStatement>, Error> {
        let location = statement.location;

        let (element, expression) = ExpressionAnalyzer::new(self.scope_stack.top())
            .analyze(statement.expression, TranslationHint::ValueExpression)?;

        let r#type = if let Some(r#type) = statement.r#type {
            let type_location = r#type.location;
            let r#type = Type::from_type_variant(&r#type.variant, self.scope_stack.top())?;
            element
                .cast(Element::Type(r#type.clone()))
                .map_err(|error| Error::Element(type_location, error))?;
            r#type
        } else {
            Type::from_element(&element, self.scope_stack.top())?
        };

        Scope::declare_variable(
            self.scope_stack.top(),
            statement.identifier.clone(),
            ScopeVariableItem::new(statement.is_mutable, r#type.clone()),
        )
        .map_err(|error| Error::Scope(location, error))?;

        Ok(GeneratorDeclarationStatement::new(
            statement.identifier.name,
            r#type,
            expression,
        ))
    }

    fn r#for(&mut self, statement: LoopStatement) -> Result<GeneratorForLoopStatement, Error> {
        let location = statement.location;
        let bounds_expression_location = statement.bounds_expression.location;

        let (range_start, range_end, index_bitlength, is_index_signed, is_inclusive) =
            match ExpressionAnalyzer::new(self.scope_stack.top()).analyze(
                statement.bounds_expression,
                TranslationHint::ValueExpression,
            )? {
                (Element::Constant(Constant::RangeInclusive(range)), _intermediate) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    true,
                ),
                (Element::Constant(Constant::Range(range)), _intermediate) => (
                    range.start,
                    range.end,
                    range.bitlength,
                    range.is_signed,
                    false,
                ),
                (element, _intermediate) => {
                    return Err(Error::LoopBoundsExpectedConstantRangeExpression {
                        location: bounds_expression_location,
                        found: element.to_string(),
                    });
                }
            };

        self.scope_stack.push();

        let index_identifier = statement.index_identifier.name.to_owned();
        Scope::declare_variable(
            self.scope_stack.top(),
            statement.index_identifier,
            ScopeVariableItem::new(false, Type::scalar(is_index_signed, index_bitlength)),
        )
        .map_err(|error| Error::Scope(location, error))?;

        let while_condition = if let Some(expression) = statement.while_condition {
            let location = expression.location;
            let (while_result, while_intermediate) =
                ExpressionAnalyzer::new(self.scope_stack.top())
                    .analyze(expression, TranslationHint::ValueExpression)?;

            match Type::from_element(&while_result, self.scope_stack.top())? {
                Type::Boolean => {}
                r#type => {
                    return Err(Error::LoopWhileExpectedBooleanCondition {
                        location,
                        found: r#type.to_string(),
                    });
                }
            }

            Some(while_intermediate)
        } else {
            None
        };

        let (_result, body) = BlockAnalyzer::analyze(self.scope_stack.top(), statement.block)?;

        self.scope_stack.pop();

        let is_reversed = range_start > range_end;
        let range_start = cmp::min(range_start, range_end.clone());
        let range_end = cmp::max(range_start.clone(), range_end);
        let iterations_count = range_end - range_start.clone();
        let mut iterations_count = iterations_count.to_usize().ok_or(Error::Element(
            bounds_expression_location,
            ElementError::Constant(ConstantError::Integer(
                IntegerConstantError::IntegerTooLarge {
                    value: iterations_count,
                    bitlength: index_bitlength,
                },
            )),
        ))?;
        if is_inclusive {
            iterations_count += 1;
        }

        Ok(GeneratorForLoopStatement::new(
            range_start,
            iterations_count,
            is_reversed,
            index_identifier,
            is_index_signed,
            index_bitlength,
            while_condition,
            body,
        ))
    }

    fn r#const(&mut self, statement: ConstStatement) -> Result<(), Error> {
        let location = statement.location;
        let type_location = statement.r#type.location;
        let expression_location = statement.expression.location;

        let (element, _intermediate) = ExpressionAnalyzer::new(self.scope_stack.top())
            .analyze(statement.expression, TranslationHint::ValueExpression)?;

        let const_type =
            Type::from_type_variant(&statement.r#type.variant, self.scope_stack.top())?;
        let constant = match element {
            Element::Constant(constant) => constant
                .cast(const_type)
                .map_err(ElementError::Constant)
                .map_err(|error| Error::Element(type_location, error))?,
            element => {
                return Err(Error::ConstantExpressionHasNonConstantElement {
                    location: expression_location,
                    found: element.to_string(),
                });
            }
        };

        Scope::declare_constant(self.scope_stack.top(), statement.identifier, constant)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn r#type(&mut self, statement: TypeStatement) -> Result<(), Error> {
        let location = statement.location;

        let r#type = Type::from_type_variant(&statement.r#type.variant, self.scope_stack.top())?;

        Scope::declare_type(self.scope_stack.top(), statement.identifier, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn r#struct(&mut self, statement: StructStatement) -> Result<(), Error> {
        let location = statement.location;

        let mut fields: Vec<(String, Type)> = Vec::with_capacity(statement.fields.len());
        for field in statement.fields.into_iter() {
            if fields
                .iter()
                .any(|(name, _type)| name == &field.identifier.name)
            {
                return Err(Error::StructureDuplicateField {
                    location: field.location,
                    type_identifier: statement.identifier.name,
                    field_name: field.identifier.name,
                });
            }
            fields.push((
                field.identifier.name,
                Type::from_type_variant(&field.r#type.variant, self.scope_stack.top())?,
            ));
        }

        let unique_id = TYPE_INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        let r#type = Type::structure(
            statement.identifier.name.clone(),
            unique_id,
            fields,
            Some(self.scope_stack.top()),
        );

        TYPE_INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());
        Scope::declare_type(self.scope_stack.top(), statement.identifier, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn r#enum(&mut self, statement: EnumStatement) -> Result<(), Error> {
        let location = statement.location;

        let unique_id = TYPE_INDEX.read().expect(crate::PANIC_MUTEX_SYNC).len();
        let r#type = Type::enumeration(
            statement.identifier.clone(),
            unique_id,
            statement.variants,
            Some(self.scope_stack.top()),
        )?;

        TYPE_INDEX
            .write()
            .expect(crate::PANIC_MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());
        Scope::declare_type(self.scope_stack.top(), statement.identifier, r#type)
            .map_err(|error| Error::Scope(location, error))?;

        Ok(())
    }

    fn r#mod(&mut self, statement: ModStatement) -> Result<(), Error> {
        let identifier_location = statement.identifier.location;
        let module = match self.dependencies.remove(statement.identifier.name.as_str()) {
            Some(module) => module,
            None => {
                return Err(Error::ModuleNotFound {
                    location: identifier_location,
                    name: statement.identifier.name,
                });
            }
        };
        Scope::declare_module(self.scope_stack.top(), statement.identifier, module)
            .map_err(|error| Error::Scope(identifier_location, error))?;

        Ok(())
    }

    fn r#use(&mut self, statement: UseStatement) -> Result<(), Error> {
        let path_location = statement.path.location;

        let path = match ExpressionAnalyzer::new(self.scope_stack.top())
            .analyze(statement.path, TranslationHint::PathExpression)?
        {
            (Element::Path(path), _intermediate) => path,
            (element, _intermediate) => {
                return Err(Error::UseExpectedPath {
                    location: path_location,
                    found: element.to_string(),
                })
            }
        };
        let item = Scope::resolve_path(self.scope_stack.top(), &path)?;
        let last_member_string = path
            .elements
            .last()
            .expect(crate::semantic::PANIC_VALIDATED_DURING_SYNTAX_ANALYSIS);
        Scope::declare_item(
            self.scope_stack.top(),
            last_member_string.to_owned().into(),
            item,
        )
        .map_err(|error| Error::Scope(last_member_string.location, error))?;

        Ok(())
    }
}
