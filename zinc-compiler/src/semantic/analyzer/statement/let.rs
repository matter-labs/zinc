//!
//! The `let` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::generator::statement::declaration::Statement as GeneratorDeclarationStatement;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::item::variant::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a variable declaration statement and returns its IR for the next compiler phase.
    ///
    pub fn analyze(
        scope: Rc<RefCell<Scope>>,
        statement: LetStatement,
    ) -> Result<Option<GeneratorDeclarationStatement>, Error> {
        let location = statement.location;

        let (element, expression) = ExpressionAnalyzer::new(scope.clone())
            .analyze(statement.expression, TranslationHint::Value)?;

        let r#type = if let Some(r#type) = statement.r#type {
            let type_location = r#type.location;
            let r#type = Type::from_type_variant(&r#type.variant, scope.clone())?;
            element
                .cast(Element::Type(r#type.clone()))
                .map_err(|error| Error::Element(type_location, error))?;
            r#type
        } else {
            Type::from_element(&element, scope.clone())?
        };

        Scope::declare_variable(
            scope,
            statement.identifier.clone(),
            ScopeVariableItem::new(statement.is_mutable, r#type.clone()),
        )?;

        Ok(GeneratorDeclarationStatement::new(
            location,
            statement.identifier.name,
            r#type,
            expression,
        ))
    }
}
