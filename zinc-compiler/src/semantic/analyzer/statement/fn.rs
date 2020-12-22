//!
//! The `fn` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_syntax::FnStatement;
use zinc_syntax::Identifier;

use crate::generator::statement::r#fn::role::Role as GeneratorFunctionRole;
use crate::generator::statement::r#fn::Statement as GeneratorFunctionStatement;
use crate::semantic::analyzer::attribute::Attribute;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::binding::Binder;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

///
/// The `fn` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines the function statement.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        mut statement: FnStatement,
    ) -> Result<(Type, Option<GeneratorFunctionStatement>), Error> {
        if let ScopeType::Contract = RefCell::borrow(&scope).r#type() {
            if statement.is_public && statement.is_constant {
                return Err(Error::EntryPointConstant {
                    location: statement.location,
                });
            }
        }

        let mut attributes = Vec::with_capacity(statement.attributes.len());
        for attribute in statement.attributes.drain(..).into_iter() {
            let attribute = Attribute::try_from(attribute)?;
            attributes.push(attribute);
        }

        if attributes.contains(&Attribute::Test) {
            return Self::test(scope, statement, attributes)
                .map(|(r#type, intermediate)| (r#type, Some(intermediate)));
        }

        if statement.is_constant {
            Self::constant(scope, statement, attributes).map(|r#type| (r#type, None))
        } else {
            Self::runtime(scope, statement, attributes)
                .map(|(r#type, intermediate)| (r#type, Some(intermediate)))
        }
    }

    ///
    /// Analyzes a runtime function statement and returns its IR for the next compiler phase.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        statement: FnStatement,
        attributes: Vec<Attribute>,
    ) -> Result<(Type, GeneratorFunctionStatement), Error> {
        let scope_type = RefCell::borrow(&scope).r#type();
        let mut scope_stack = if scope_type.is_implementation() {
            let alias_identifier =
                Identifier::new(statement.location, Keyword::SelfUppercase.to_string());
            let item = RefCell::borrow(&scope).resolve_item(&alias_identifier, false)?;

            let mut scope_stack = ScopeStack::new(
                RefCell::borrow(&scope)
                    .parent()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            );
            scope_stack.push(Some(statement.identifier.name.clone()), ScopeType::Function);
            Scope::define_item(scope_stack.top(), alias_identifier, item)?;
            scope_stack
        } else {
            let mut scope_stack = ScopeStack::new(scope);
            scope_stack.push(Some(statement.identifier.name.clone()), ScopeType::Function);
            scope_stack
        };

        let bindings = Binder::bind_arguments(statement.argument_bindings, scope_stack.top())?;

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        if !expected_type.is_instantiatable(false) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement
                    .return_type
                    .map(|r#type| r#type.location)
                    .unwrap_or(statement.location),
                found: expected_type.to_string(),
            });
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

        let (result, intermediate) =
            BlockAnalyzer::analyze(scope_stack.top(), statement.body, TranslationRule::Value)?;
        scope_stack.pop();

        let result_type = Type::from_element(&result, scope_stack.top())?;
        if expected_type != result_type {
            return Err(Error::FunctionReturnType {
                location: return_expression_location,
                function: statement.identifier.name.clone(),
                expected: expected_type.to_string(),
                found: result_type.to_string(),
                reference: statement
                    .return_type
                    .map(|r#type| r#type.location)
                    .unwrap_or(statement.location),
            });
        }

        let is_in_dependency = scope_stack
            .top()
            .borrow()
            .entry()
            .map(|(_project, is_dependency)| is_dependency)
            .unwrap_or_default();
        let is_method = bindings
            .first()
            .map(|binding| matches!(binding.r#type, Type::Contract(_)))
            .unwrap_or_default();

        let role = match scope_type {
            ScopeType::Contract if statement.is_public && is_method && !is_in_dependency => {
                GeneratorFunctionRole::ContractMethodEntry
            }
            ScopeType::Contract if statement.is_public => match expected_type {
                Type::Contract(ref contract) => GeneratorFunctionRole::ContractConstuctor {
                    project: contract.project.to_owned(),
                },
                _ => GeneratorFunctionRole::Ordinar,
            },
            _ if statement.identifier.name.as_str()
                == zinc_const::source::FUNCTION_MAIN_IDENTIFIER =>
            {
                GeneratorFunctionRole::CircuitEntry
            }
            _ => GeneratorFunctionRole::Ordinar,
        };

        let is_mutable = bindings
            .first()
            .map(|binding| binding.is_mutable)
            .unwrap_or_default();

        let (r#type, type_id) = Type::runtime_function(
            statement.location,
            statement.identifier.name.clone(),
            bindings.clone(),
            expected_type.clone(),
        );

        let intermediate = GeneratorFunctionStatement::new(
            statement.location,
            statement.identifier.name,
            is_mutable,
            bindings,
            intermediate,
            expected_type,
            type_id,
            role,
            attributes,
        );

        Ok((r#type, intermediate))
    }

    ///
    /// Analyzes a constant function statement.
    ///
    fn constant(
        scope: Rc<RefCell<Scope>>,
        statement: FnStatement,
        _attributes: Vec<Attribute>,
    ) -> Result<Type, Error> {
        let mut scope_stack = ScopeStack::new(scope);
        scope_stack.push(Some(statement.identifier.name.clone()), ScopeType::Function);

        let bindings = Binder::bind_arguments(statement.argument_bindings, scope_stack.top())?;

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        if !expected_type.is_instantiatable(false) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement
                    .return_type
                    .map(|r#type| r#type.location)
                    .unwrap_or(statement.location),
                found: expected_type.to_string(),
            });
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

        let (result, _intermediate) = BlockAnalyzer::analyze(
            scope_stack.top(),
            statement.body.clone(),
            TranslationRule::Value,
        )?;
        scope_stack.pop();

        let result_type = Type::from_element(&result, scope_stack.top())?;
        if expected_type != result_type {
            return Err(Error::FunctionReturnType {
                location: return_expression_location,
                function: statement.identifier.name.clone(),
                expected: expected_type.to_string(),
                found: result_type.to_string(),
                reference: statement
                    .return_type
                    .map(|r#type| r#type.location)
                    .unwrap_or(statement.location),
            });
        }

        Ok(Type::constant_function(
            statement.location,
            statement.identifier.name.clone(),
            bindings,
            expected_type,
            statement.body,
        ))
    }

    ///
    /// Analyzes a test function statement and returns its IR for the next compiler phase.
    ///
    fn test(
        scope: Rc<RefCell<Scope>>,
        statement: FnStatement,
        attributes: Vec<Attribute>,
    ) -> Result<(Type, GeneratorFunctionStatement), Error> {
        let location = statement.location;

        let mut scope_stack = ScopeStack::new(scope);

        if !RefCell::borrow(&scope_stack.top()).r#type().is_module() {
            return Err(Error::UnitTestBeyondModuleScope {
                location,
                function: statement.identifier.name,
            });
        }

        if statement.is_public {
            return Err(Error::UnitTestPublicForbidden {
                location,
                function: statement.identifier.name,
            });
        }

        if statement.is_constant {
            return Err(Error::UnitTestConstantForbidden {
                location,
                function: statement.identifier.name,
            });
        }

        if !statement.argument_bindings.is_empty() {
            return Err(Error::UnitTestCannotHaveArguments {
                location,
                function: statement.identifier.name,
            });
        }

        if statement.return_type.is_some() {
            return Err(Error::UnitTestCannotReturnValue {
                location,
                function: statement.identifier.name,
            });
        }

        scope_stack.push(Some(statement.identifier.name.clone()), ScopeType::Function);
        let (_result, intermediate) =
            BlockAnalyzer::analyze(scope_stack.top(), statement.body, TranslationRule::Value)?;
        scope_stack.pop();

        let (r#type, type_id) =
            Type::test_function(statement.location, statement.identifier.name.clone());

        let intermediate = GeneratorFunctionStatement::new(
            location,
            statement.identifier.name,
            false,
            vec![],
            intermediate,
            Type::Unit(None),
            type_id,
            GeneratorFunctionRole::UnitTest,
            attributes,
        );

        Ok((r#type, intermediate))
    }
}
