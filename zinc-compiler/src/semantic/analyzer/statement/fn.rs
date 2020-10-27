//!
//! The `fn` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use zinc_syntax::FnStatement;

use crate::generator::statement::r#fn::Statement as GeneratorFunctionStatement;
use crate::semantic::analyzer::attribute::Attribute;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::binding::Binder;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;

///
/// The context lets the analyzer know where the function is declared.
///
#[derive(Debug, Clone, Copy)]
pub enum Context {
    /// The module root namespace.
    Module,
    /// The type implementation namespace.
    Implementation,
    /// The contract definition namespace.
    Contract,
}

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
        context: Context,
    ) -> Result<(Type, Option<GeneratorFunctionStatement>), Error> {
        if let Context::Contract = context {
            if statement.is_public && statement.is_constant {
                return Err(Error::EntryPointConstant {
                    location: statement.location,
                });
            }
        }

        let mut attributes = Vec::with_capacity(statement.attributes.len());
        for attribute in statement.attributes.drain(..).into_iter() {
            let attribute = Attribute::try_from(attribute).map_err(Error::Attribute)?;
            attributes.push(attribute);
        }

        if attributes.contains(&Attribute::Test) {
            return Self::test(scope, statement, context, attributes)
                .map(|(r#type, intermediate)| (r#type, Some(intermediate)));
        }

        if statement.is_constant {
            Self::constant(scope, statement, context, attributes).map(|r#type| (r#type, None))
        } else {
            Self::runtime(scope, statement, context, attributes)
                .map(|(r#type, intermediate)| (r#type, Some(intermediate)))
        }
    }

    ///
    /// Analyzes a runtime function statement and returns its IR for the next compiler phase.
    ///
    fn runtime(
        scope: Rc<RefCell<Scope>>,
        statement: FnStatement,
        context: Context,
        attributes: Vec<Attribute>,
    ) -> Result<(Type, GeneratorFunctionStatement), Error> {
        let mut scope_stack = ScopeStack::new(scope);
        scope_stack.push(Some(statement.identifier.name.clone()));

        let bindings =
            Binder::bind_arguments(statement.argument_bindings, context, scope_stack.top())?;

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        if !expected_type.is_instantiatable(false) {
            return Err(Error::Element(ElementError::Type(
                TypeError::InstantiationForbidden {
                    location: statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                    found: expected_type.to_string(),
                },
            )));
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
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::ReturnType {
                    location: return_expression_location,
                    function: statement.identifier.name.clone(),
                    expected: expected_type.to_string(),
                    found: result_type.to_string(),
                    reference: statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                },
            ))));
        }

        let (is_main, is_contract_entry) = if let Context::Contract = context {
            (false, statement.is_public)
        } else {
            (
                statement.identifier.name.as_str() == zinc_const::source::FUNCTION_MAIN_IDENTIFIER,
                false,
            )
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
            is_contract_entry,
            is_main,
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
        context: Context,
        _attributes: Vec<Attribute>,
    ) -> Result<Type, Error> {
        let mut scope_stack = ScopeStack::new(scope);
        scope_stack.push(Some(statement.identifier.name.clone()));

        let bindings =
            Binder::bind_arguments(statement.argument_bindings, context, scope_stack.top())?;

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        if !expected_type.is_instantiatable(false) {
            return Err(Error::Element(ElementError::Type(
                TypeError::InstantiationForbidden {
                    location: statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                    found: expected_type.to_string(),
                },
            )));
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
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::ReturnType {
                    location: return_expression_location,
                    function: statement.identifier.name.clone(),
                    expected: expected_type.to_string(),
                    found: result_type.to_string(),
                    reference: statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                },
            ))));
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
        context: Context,
        attributes: Vec<Attribute>,
    ) -> Result<(Type, GeneratorFunctionStatement), Error> {
        let location = statement.location;

        let mut scope_stack = ScopeStack::new(scope);

        match context {
            Context::Module => {}
            _context => {
                return Err(Error::Element(ElementError::Type(TypeError::Function(
                    FunctionError::Test(TestFunctionError::BeyondModuleScope {
                        location,
                        function: statement.identifier.name,
                    }),
                ))))
            }
        }

        if statement.is_public {
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::Test(TestFunctionError::PublicForbidden {
                    location,
                    function: statement.identifier.name,
                }),
            ))));
        }

        if statement.is_constant {
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::Test(TestFunctionError::ConstantForbidden {
                    location,
                    function: statement.identifier.name,
                }),
            ))));
        }

        if !statement.argument_bindings.is_empty() {
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::Test(TestFunctionError::CannotHaveArguments {
                    location,
                    function: statement.identifier.name,
                }),
            ))));
        }

        if statement.return_type.is_some() {
            return Err(Error::Element(ElementError::Type(TypeError::Function(
                FunctionError::Test(TestFunctionError::CannotReturnValue {
                    location,
                    function: statement.identifier.name,
                }),
            ))));
        }

        scope_stack.push(Some(statement.identifier.name.clone()));
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
            false,
            false,
            attributes,
        );

        Ok((r#type, intermediate))
    }
}
