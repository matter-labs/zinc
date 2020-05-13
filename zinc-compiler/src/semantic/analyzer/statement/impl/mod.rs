//!
//! The `impl` statement semantic analyzer.
//!

mod tests;

pub mod error;

use std::cell::RefCell;
use std::rc::Rc;

use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::Analyzer as StatementAnalyzer;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

pub struct Analyzer {}

impl Analyzer {
    ///
    /// Analyzes an implementation statement and returns its IR for the next compiler phase.
    ///
    pub fn analyze(scope: Rc<RefCell<Scope>>, statement: ImplStatement) -> Result<Type, Error> {
        let identifier_location = statement.identifier.location;

        let (r#type, type_scope) = match Scope::resolve_item(scope, &statement.identifier, true)? {
            ScopeItem::Type(r#type) => {
                let r#type = r#type.resolve()?;
                let scope = match r#type {
                    Type::Structure(ref inner) => inner.scope.to_owned(),
                    Type::Enumeration(ref inner) => inner.scope.to_owned(),
                    _type => {
                        return Err(Error::Statement(StatementError::Impl(
                            ImplStatementError::ExpectedStructureOrEnumeration {
                                location: identifier_location,
                                found: statement.identifier.name,
                            },
                        )))
                    }
                };
                (r#type, scope)
            }
            _item => {
                return Err(Error::Statement(StatementError::Impl(
                    ImplStatementError::ExpectedStructureOrEnumeration {
                        location: identifier_location,
                        found: statement.identifier.name,
                    },
                )));
            }
        };

        StatementAnalyzer::implementation(statement, type_scope)?;

        Ok(r#type)
    }
}
