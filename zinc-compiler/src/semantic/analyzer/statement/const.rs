//!
//! The `const` statement semantic analyzer.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_syntax::ConstStatement;

use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;

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
        if !const_type.is_instantiatable(false) {
            return Err(Error::TypeInstantiationForbidden {
                location: statement.location,
                found: const_type.to_string(),
            });
        }

        let (constant, _intermediate) = match element {
            Element::Constant(constant) => constant.cast(const_type)?,
            element => {
                return Err(Error::ExpressionNonConstantElement {
                    location: expression_location,
                    found: element.to_string(),
                });
            }
        };

        Ok(constant)
    }
}
