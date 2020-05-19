//!
//! The `let` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::declaration::Statement as GeneratorDeclarationStatement;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variable::memory_type::MemoryType;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a variable and returns its IR for the next compiler phase.
    ///
    pub fn define(
        scope: Rc<RefCell<Scope>>,
        statement: LetStatement,
    ) -> Result<Option<GeneratorDeclarationStatement>, Error> {
        let (element, expression) = ExpressionAnalyzer::new(scope.clone(), TranslationRule::Value)
            .analyze(statement.expression)?;

        let r#type = if let Some(r#type) = statement.r#type {
            let r#type = Type::from_syntax_type(r#type, scope.clone())?;
            element
                .cast(Element::Type(r#type.clone()))
                .map_err(Error::Element)?;
            r#type
        } else {
            Type::from_element(&element, scope.clone())?
        };

        Scope::define_variable(
            scope,
            statement.identifier.clone(),
            statement.is_mutable,
            r#type.clone(),
            MemoryType::Stack,
        )?;

        Ok(GeneratorDeclarationStatement::new(
            statement.location,
            statement.identifier.name,
            r#type,
            expression,
        ))
    }
}
