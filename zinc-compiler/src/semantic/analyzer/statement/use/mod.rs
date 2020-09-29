//!
//! The `use` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

pub mod error;

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::expression::Analyzer as ExpressionAnalyzer;
use crate::semantic::analyzer::rule::Rule as TranslationRule;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#use::error::Error as UseStatementError;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

///
/// The `use` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Defines an item imported by the compile-time only `use` statement.
    ///
    pub fn define(scope: Rc<RefCell<Scope>>, statement: UseStatement) -> Result<(), Error> {
        let path_location = statement.path.location;

        let path = match ExpressionAnalyzer::new(scope.clone(), TranslationRule::Path)
            .analyze(statement.path)?
        {
            (Element::Path(path), _intermediate) => path,
            (element, _intermediate) => {
                return Err(Error::Statement(StatementError::Use(
                    UseStatementError::ExpectedPath {
                        location: path_location,
                        found: element.to_string(),
                    },
                )))
            }
        };

        let mut item = Scope::resolve_path(scope.clone(), &path)?.borrow().clone();
        item.set_not_associated();
        let identifier = match statement.alias_identifier {
            Some(alias_identifier) => alias_identifier,
            None => path.last().to_owned(),
        };
        Scope::define_item(scope, identifier, item.wrap())?;

        Ok(())
    }
}
