//!
//! The `impl` statement semantic analyzer.
//!

#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_syntax::ImplStatement;
use zinc_syntax::ImplementationLocalStatement;

use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;
use crate::semantic::scope::item::r#type::state::State as ScopeTypeItemState;
use crate::semantic::scope::item::r#type::statement::Statement as TypeStatementVariant;
use crate::semantic::scope::item::r#type::statement::Statement as ScopeTypeItemStatement;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

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
                        return Err(Error::ImplStatementExpectedStructureOrEnumeration {
                            location: identifier_location,
                            found: statement.identifier.name,
                        })
                    }
                },
                Some(ScopeTypeItemState::Defined {
                    inner: ref r#type, ..
                }) => match r#type {
                    Type::Structure(ref inner) => inner.scope.to_owned(),
                    Type::Enumeration(ref inner) => inner.scope.to_owned(),
                    ref _type => {
                        return Err(Error::ImplStatementExpectedStructureOrEnumeration {
                            location: identifier_location,
                            found: statement.identifier.name,
                        })
                    }
                },
                None => {
                    return Err(Error::ScopeReferenceLoop {
                        location: identifier_location,
                    });
                }
            },
            ref _item => {
                return Err(Error::ImplStatementExpectedStructureOrEnumeration {
                    location: identifier_location,
                    found: statement.identifier.name,
                });
            }
        };

        Scope::insert_item(scope.clone(), Keyword::SelfUppercase.to_string(), item);

        for hoisted_statement in statement.statements.into_iter() {
            match hoisted_statement {
                ImplementationLocalStatement::Const(statement) => {
                    Scope::declare_constant(scope.clone(), statement)?;
                }
                ImplementationLocalStatement::Fn(statement) => {
                    Scope::declare_type(scope.clone(), TypeStatementVariant::Fn(statement))?;
                }
                ImplementationLocalStatement::Empty(_location) => {}
            }
        }

        Ok(scope)
    }
}
