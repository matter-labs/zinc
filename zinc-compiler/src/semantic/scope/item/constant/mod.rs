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
    /// The location, where the constant is declared.
    pub location: Location,
    /// The unique constant ID, allocated upon declaration.
    pub item_id: usize,
    /// The definition state, which is either `declared` or `defined`.
    pub state: RefCell<Option<State>>,
    /// Whether the constant is associated with some implementation or smart contract definition.
    pub is_associated: bool,
}

impl Constant {
    ///
    /// Creates an declared constant, which must be defined during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    pub fn new_declared(
        location: Location,
        inner: ConstStatement,
        scope: Rc<RefCell<Scope>>,
        is_associated: bool,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("constant {}", inner.identifier.name));

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Declared { inner, scope })),
            is_associated,
        }
    }

    ///
    /// Creates a defined constant, which is ready to be used from anywhere.
    ///
    pub fn new_defined(location: Location, inner: ConstantElement, is_associated: bool) -> Self {
        let item_id = ITEM_INDEX.next(inner.to_string());

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Defined { inner })),
            is_associated,
        }
    }

    ///
    /// Defines the declared constant.
    ///
    /// The method is able to detect reference loops. It happens naturally when the method
    /// is reentered before the item being defined is put back into `variant`, which means that
    /// the item is taken twice during its resolution process.
    ///
    pub fn define(&self) -> Result<ConstantElement, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Declared { inner, scope }) => {
                let defined = ConstStatementAnalyzer::define(scope, inner)?;
                self.state.replace(Some(State::Defined {
                    inner: defined.clone(),
                }));

                Ok(defined)
            }
            Some(State::Defined { inner }) => {
                self.state.replace(Some(State::Defined {
                    inner: inner.clone(),
                }));

                Ok(inner)
            }
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location,
            })),
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state.borrow().as_ref() {
            Some(State::Declared { inner, .. }) => write!(f, "{}", inner.identifier.name),
            Some(State::Defined { inner, .. }) => write!(f, "{}", inner),
            None => write!(f, "<resolving {}>", self.location),
        }
    }
}
