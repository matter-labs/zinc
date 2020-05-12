//!
//! The semantic analyzer scope constant item.
//!

pub mod state;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::r#const::Analyzer as ConstStatementAnalyzer;
use crate::semantic::element::constant::Constant as ConstantElement;
use crate::semantic::error::Error;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use crate::semantic::scope::Scope;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;

use self::state::State;

///
/// The constant item, declared using a `const` statement.
///
#[derive(Debug, Clone)]
pub struct Constant {
    pub location: Location,
    pub item_id: usize,
    pub state: RefCell<Option<State>>,
}

impl Constant {
    ///
    /// Creates an unresolved constant, which must be resolved during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    pub fn new_unresolved(
        location: Location,
        inner: ConstStatement,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("constant {}", inner.identifier.name));

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Unresolved { inner, scope })),
        }
    }

    ///
    /// Creates a resolved constant, which is ready to be used from anywhere.
    ///
    /// Is used for non-module level constants which are not hoisted.
    ///
    pub fn new_resolved(location: Location, inner: ConstantElement) -> Self {
        let item_id = ITEM_INDEX.next(inner.to_string());

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Resolved { inner })),
        }
    }

    pub fn resolve(&self) -> Result<ConstantElement, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Unresolved { inner, scope }) => {
                let resolved = ConstStatementAnalyzer::analyze(scope, inner)?;
                self.state.replace(Some(State::Resolved {
                    inner: resolved.clone(),
                }));

                Ok(resolved)
            }
            Some(State::Resolved { inner }) => {
                self.state.replace(Some(State::Resolved {
                    inner: inner.clone(),
                }));

                Ok(inner)
            }
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location,
            })),
        }
    }

    pub fn is_resolved(&self) -> bool {
        match self.state.borrow().as_ref() {
            Some(State::Resolved { .. }) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state.borrow().as_ref() {
            Some(State::Unresolved { inner, .. }) => write!(f, "{}", inner.identifier.name),
            Some(State::Resolved { inner, .. }) => write!(f, "{}", inner),
            None => write!(f, "<resolving {}>", self.location),
        }
    }
}
