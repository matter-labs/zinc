//!
//! The `const` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes a compile-time only constant declaration statement.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: ConstStatement) -> Result<(), Error> {
        let type_location = statement.r#type.location;
        let expression_location = statement.expression.location;

        let (element, _intermediate) = ExpressionAnalyzer::new(scope.clone())
            .analyze(statement.expression, TranslationHint::Value)?;

        let const_type = Type::from_type_variant(&statement.r#type.variant, scope.clone())?;
        let constant = match element {
            Element::Constant(constant) => constant
                .cast(const_type)
                .map_err(ElementError::Constant)
                .map_err(|error| Error::Element(type_location, error))?,
            element => {
                return Err(Error::Expression(ExpressionError::NonConstantElement {
                    location: expression_location,
                    found: element.to_string(),
                }));
            }
        };

        Scope::declare_constant(scope, statement.identifier, constant)?;

        Ok(())
    }
}
