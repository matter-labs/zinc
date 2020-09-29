//!
//! The `impl` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

pub mod error;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#fn::Context as FnStatementAnalyzerContext;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::scope::item::r#type::state::State as ScopeTypeItemState;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::item::r#type::statement::Statement as ScopeTypeItemStatement;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;
use crate::syntax::tree::statement::r#impl::Statement as ImplStatement;

///
/// The `impl` statement semantic analyzer.
///
pub struct Analyzer {}

impl Analyzer {
    ///
    /// Acquires the type being implemented and declares the hoisted items.
    ///
    /// Also declares the `Self` alias for the type being implemented.
    ///
    pub fn declare(
        scope: Rc<RefCell<Scope>>,
        statement: ImplStatement,
    ) -> Result<Rc<RefCell<Scope>>, Error> {
        let identifier_location = statement.identifier.location;

        let item = scope.borrow().resolve_item(&statement.identifier, true)?;

        let scope = match *item.borrow() {
            ScopeItem::Type(ScopeTypeItem { ref state, .. }) => match state.borrow().as_ref() {
                Some(ScopeTypeItemState::Declared {
                    ref inner,
                    ref scope,
                }) => match inner {
                    ScopeTypeItemStatement::Struct(_) => scope.to_owned(),
                    ScopeTypeItemStatement::Enum(_) => scope.to_owned(),
                    ref _statement => {
                        return Err(Error::Statement(StatementError::Impl(
                            ImplStatementError::ExpectedStructureOrEnumeration {
                                location: identifier_location,
                                found: statement.identifier.name,
                            },
                        )))
                    }
                },
                Some(ScopeTypeItemState::Defined {
                    inner: ref r#type, ..
                }) => match r#type {
                    Type::Structure(ref inner) => inner.scope.to_owned(),
                    Type::Enumeration(ref inner) => inner.scope.to_owned(),
                    ref _type => {
                        return Err(Error::Statement(StatementError::Impl(
                            ImplStatementError::ExpectedStructureOrEnumeration {
                                location: identifier_location,
                                found: statement.identifier.name,
                            },
                        )))
                    }
                },
                None => {
                    return Err(Error::Scope(ScopeError::ReferenceLoop {
                        location: identifier_location,
                    }))
                }
            },
            ref _item => {
                return Err(Error::Statement(StatementError::Impl(
                    ImplStatementError::ExpectedStructureOrEnumeration {
                        location: identifier_location,
                        found: statement.identifier.name,
                    },
                )));
            }
        };

        Scope::insert_item(scope.clone(), Keyword::SelfUppercase.to_string(), item);

        for hoisted_statement in statement.statements.into_iter() {
            match hoisted_statement {
                ImplementationLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement, true)?;
                }
                ImplementationLocalStatement::Fn(statement) => {
                    Scope::declare_type(
                        scope.clone(),
                        TypeStatementVariant::Fn(
                            statement,
                            FnStatementAnalyzerContext::Implementation,
                        ),
                        true,
                    )?;
                }
                ImplementationLocalStatement::Empty(_location) => {}
            }
        }

        Ok(scope)
    }
}
