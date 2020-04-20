//!
//! The `fn` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::function::Statement as GeneratorFunctionStatement;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::semantic::analyzer::expression::block::Analyzer as BlockAnalyzer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::function::user::Function as UserDefinedFunctionType;
use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::r#type::INDEX as TYPE_INDEX;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::stack::Stack as ScopeStack;
use crate::semantic::scope::Scope;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::pattern_binding::variant::Variant as BindingPatternVariant;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a function statement and returns its IR for the next compiler phase.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: FnStatement,
    ) -> Result<GeneratorFunctionStatement, Error> {
        let location = statement.location;

        let mut scope_stack = ScopeStack::new(scope);

        let mut arguments = Vec::with_capacity(statement.argument_bindings.len());
        for (index, argument_binding) in statement.argument_bindings.iter().enumerate() {
            let identifier = match argument_binding.variant {
                BindingPatternVariant::Binding { ref identifier, .. } => identifier.name.to_owned(),
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias { .. } => {
                    if index != 0 {
                        return Err(Error::Element(
                            statement.identifier.location,
                            ElementError::Type(TypeError::Function(
                                FunctionTypeError::function_method_self_not_first(
                                    statement.identifier.name.clone(),
                                    index + 1,
                                    argument_binding.location,
                                ),
                            )),
                        ));
                    }

                    Keyword::SelfLowercase.to_string()
                }
            };

            arguments.push((
                identifier,
                Type::from_type_variant(&argument_binding.r#type.variant, scope_stack.top())?,
            ));
        }

        let expected_type = match statement.return_type {
            Some(ref r#type) => Type::from_type_variant(&r#type.variant, scope_stack.top())?,
            None => Type::unit(),
        };

        let unique_id = TYPE_INDEX.read().expect(crate::panic::MUTEX_SYNC).len();
        let function_type = UserDefinedFunctionType::new(
            statement.identifier.name.clone(),
            unique_id,
            arguments.clone(),
            expected_type.clone(),
        );
        let r#type = Type::Function(FunctionType::UserDefined(function_type));

        TYPE_INDEX
            .write()
            .expect(crate::panic::MUTEX_SYNC)
            .insert(unique_id, r#type.to_string());
        Scope::declare_type(scope_stack.top(), statement.identifier.clone(), r#type)?;

        scope_stack.push();
        for argument_binding in statement.argument_bindings.into_iter() {
            match argument_binding.variant {
                BindingPatternVariant::Binding {
                    identifier,
                    is_mutable,
                } => {
                    let r#type = Type::from_type_variant(
                        &argument_binding.r#type.variant,
                        scope_stack.top(),
                    )?;

                    Scope::declare_variable(
                        scope_stack.top(),
                        identifier,
                        ScopeVariableItem::new(is_mutable, r#type),
                    )?;
                }
                BindingPatternVariant::Wildcard => continue,
                BindingPatternVariant::SelfAlias {
                    location,
                    is_mutable,
                } => {
                    let identifier = Identifier::new(location, Keyword::SelfLowercase.to_string());
                    let r#type = Type::from_type_variant(
                        &argument_binding.r#type.variant,
                        scope_stack.top(),
                    )?;

                    Scope::declare_variable(
                        scope_stack.top(),
                        identifier,
                        ScopeVariableItem::new(is_mutable, r#type),
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
        let (result, body) = BlockAnalyzer::analyze(scope_stack.top(), statement.body)?;
        scope_stack.pop();

        let result_type = Type::from_element(&result, scope_stack.top())?;
        if expected_type != result_type {
            return Err(Error::Element(
                return_expression_location,
                ElementError::Type(TypeError::Function(FunctionTypeError::return_type(
                    statement.identifier.name.clone(),
                    expected_type.to_string(),
                    result_type.to_string(),
                    statement
                        .return_type
                        .map(|r#type| r#type.location)
                        .unwrap_or(statement.location),
                ))),
            ));
        }

        let is_main = statement.identifier.name.as_str() == crate::FUNCTION_MAIN_IDENTIFIER;

        Ok(GeneratorFunctionStatement::new(
            location,
            statement.identifier.name,
            arguments,
            body,
            expected_type,
            unique_id,
            statement.is_public,
            is_main,
        ))
    }
}
