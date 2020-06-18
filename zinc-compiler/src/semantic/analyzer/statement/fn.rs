//!
//! The `fn` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use crate::generator::statement::r#fn::Statement as GeneratorFunctionStatement;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::semantic::analyzer::attribute::Attribute;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::memory_type::MemoryType;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

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
        let location = statement.location;

        let mut scope_stack = ScopeStack::new(scope);

        let mut arguments = Vec::with_capacity(statement.argument_bindings.len());
        for (index, argument_binding) in statement.argument_bindings.iter().enumerate() {
            let identifier = match argument_binding.variant {
                BindingPatternVariant::Binding { ref identifier, .. } => identifier.name.to_owned(),
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias { .. } => {
                    if index != 0 {
                        return Err(Error::Element(ElementError::Type(TypeError::Function(
                            FunctionError::FunctionMethodSelfNotFirst {
                                location: statement.identifier.location,
                                function: statement.identifier.name.clone(),
                                position: index + 1,
                                reference: argument_binding.location,
                            },
                        ))));
                    }

                    Keyword::SelfLowercase.to_string()
                }
            };

            arguments.push((
                identifier,
                Type::try_from_syntax(argument_binding.r#type.to_owned(), scope_stack.top())?,
            ));
        }

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        scope_stack.push(Some(statement.identifier.name.clone()));
        for argument_binding in statement.argument_bindings.into_iter() {
            match argument_binding.variant {
                BindingPatternVariant::Binding {
                    identifier,
                    is_mutable,
                } => {
                    let r#type = Type::try_from_syntax(argument_binding.r#type, scope_stack.top())?;

                    Scope::define_variable(
                        scope_stack.top(),
                        identifier,
                        is_mutable,
                        r#type,
                        MemoryType::Stack,
                    )?;
                }
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias {
                    location,
                    is_mutable,
                } => {
                    let identifier = Identifier::new(location, Keyword::SelfLowercase.to_string());
                    let r#type = Type::try_from_syntax(argument_binding.r#type, scope_stack.top())?;

                    let memory_type = match context {
                        Context::Contract => MemoryType::ContractInstance,
                        Context::Module => MemoryType::Stack,
                        Context::Implementation => MemoryType::Stack,
                    };

                    Scope::define_variable(
                        scope_stack.top(),
                        identifier,
                        is_mutable,
                        r#type,
                        memory_type,
                    )?;
                }
            }
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

        let is_main = statement.identifier.name.as_str() == crate::FUNCTION_MAIN_IDENTIFIER;

        let is_contract_entry = if let Context::Contract = context {
            statement.is_public
        } else {
            false
        };

        let (r#type, type_id) = Type::runtime_function(
            statement.location,
            statement.identifier.name.clone(),
            arguments.clone(),
            expected_type.clone(),
        );

        let intermediate = GeneratorFunctionStatement::new(
            location,
            statement.identifier.name,
            arguments,
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

        let mut arguments = Vec::with_capacity(statement.argument_bindings.len());
        for (index, argument_binding) in statement.argument_bindings.iter().enumerate() {
            let identifier = match argument_binding.variant {
                BindingPatternVariant::Binding { ref identifier, .. } => identifier.name.to_owned(),
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias { .. } => {
                    if index != 0 {
                        return Err(Error::Element(ElementError::Type(TypeError::Function(
                            FunctionError::FunctionMethodSelfNotFirst {
                                location: statement.identifier.location,
                                function: statement.identifier.name.clone(),
                                position: index + 1,
                                reference: argument_binding.location,
                            },
                        ))));
                    }

                    Keyword::SelfLowercase.to_string()
                }
            };

            arguments.push((
                identifier,
                Type::try_from_syntax(argument_binding.r#type.to_owned(), scope_stack.top())?,
            ));
        }

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::try_from_syntax(r#type.to_owned(), scope_stack.top())?,
            None => Type::unit(None),
        };

        scope_stack.push(Some(statement.identifier.name.clone()));
        for argument_binding in statement.argument_bindings.into_iter() {
            match argument_binding.variant {
                BindingPatternVariant::Binding {
                    identifier,
                    is_mutable,
                } => {
                    let r#type = Type::try_from_syntax(argument_binding.r#type, scope_stack.top())?;

                    Scope::define_variable(
                        scope_stack.top(),
                        identifier,
                        is_mutable,
                        r#type,
                        MemoryType::Stack,
                    )?;
                }
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias {
                    location,
                    is_mutable,
                } => {
                    let identifier = Identifier::new(location, Keyword::SelfLowercase.to_string());
                    let r#type = Type::try_from_syntax(argument_binding.r#type, scope_stack.top())?;

                    let memory_type = match context {
                        Context::Contract => MemoryType::ContractInstance,
                        Context::Module => MemoryType::Stack,
                        Context::Implementation => MemoryType::Stack,
                    };

                    Scope::define_variable(
                        scope_stack.top(),
                        identifier,
                        is_mutable,
                        r#type,
                        memory_type,
                    )?;
                }
            }
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
            arguments,
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
