//!
//! The `const` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

///
/// The `const` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines a compile-time only constant item.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: ConstStatement) -> Result<Constant, Error> {
        let expression_location = statement.expression.location;

        let (element, _intermediate) =
            ExpressionAnalyzer::new(scope.clone(), TranslationRule::Constant)
                .analyze(statement.expression)?;

        let const_type = Type::try_from_syntax(statement.r#type, scope)?;
        let (constant, _intermediate) = match element {
            Element::Constant(constant) => constant
                .cast(const_type)
                .map_err(ElementError::Constant)
                .map_err(Error::Element)?,
            element => {
                return Err(Error::Expression(ExpressionError::NonConstantElement {
                    location: expression_location,
                    found: element.to_string(),
                }));
            }
        };

        Ok(constant)
    }
}
