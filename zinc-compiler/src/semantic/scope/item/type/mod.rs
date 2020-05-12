//!
//! The semantic analyzer scope type item.
//!

pub mod index;
pub mod state;
pub mod statement;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::contract::Analyzer as ContractStatementAnalyzer;
use crate::semantic::analyzer::statement::r#enum::Analyzer as EnumStatementAnalyzer;
use crate::semantic::analyzer::statement::r#fn::Analyzer as FnStatementAnalyzer;
use crate::semantic::analyzer::statement::r#struct::Analyzer as StructStatementAnalyzer;
use crate::semantic::analyzer::statement::r#type::Analyzer as TypeStatementAnalyzer;
use crate::semantic::element::r#type::Type as TypeElement;
use crate::semantic::error::Error;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use crate::semantic::scope::Scope;

use self::state::State;
use self::statement::Statement as TypeStatementVariant;

///
/// The type item, declared using a `type`, `struct`, `enum`, or another statement.
///
#[derive(Debug, Clone)]
pub struct Type {
    pub location: Option<Location>,
    pub item_id: usize,
    pub state: RefCell<Option<State>>,
}

impl Type {
    ///
    /// Creates an unresolved type, which must be resolved during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    pub fn new_unresolved(
        location: Option<Location>,
        inner: TypeStatementVariant,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        let item_id = ITEM_INDEX.next(format!("type {}", inner.identifier().name));

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Unresolved { inner, scope })),
        }
    }

    ///
    /// Creates a resolved type, which is ready to be used from anywhere.
    ///
    /// Is used for items which are not hoisted.
    ///
    pub fn new_resolved(
        location: Option<Location>,
        inner: TypeElement,
        is_self_alias: bool,
        intermediate: Option<GeneratorStatement>,
    ) -> Self {
        let mut title = inner.to_string();
        if is_self_alias {
            title.push_str(" (Self)")
        }
        let item_id = ITEM_INDEX.next(title);

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Resolved {
                inner,
                intermediate,
            })),
        }
    }

    ///
    /// Useful method to declare a built-in type without a `location` or `intermediate` represenation.
    ///
    pub fn new_built_in(inner: TypeElement) -> Self {
        let item_id = ITEM_INDEX.next(inner.to_string());

        Self {
            location: None,
            item_id,
            state: RefCell::new(Some(State::Resolved {
                inner,
                intermediate: None,
            })),
        }
    }

    ///
    /// Analyzes the unresolved item and puts the resolved one in its place.
    ///
    /// The method is able to detect reference loops. It happens out of the box when the
    /// method is reentered before the resolved item is put into `variant`, which means that
    /// the item is taken twice during resolution.
    ///
    pub fn resolve(&self) -> Result<TypeElement, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Unresolved { inner, scope }) => {
                let (r#type, intermediate) = match inner {
                    TypeStatementVariant::Type(inner) => {
                        (TypeStatementAnalyzer::analyze(scope, inner)?, None)
                    }
                    TypeStatementVariant::Struct(inner) => {
                        (StructStatementAnalyzer::analyze(scope, inner)?, None)
                    }
                    TypeStatementVariant::Enum(inner) => {
                        (EnumStatementAnalyzer::analyze(scope, inner)?, None)
                    }
                    TypeStatementVariant::Fn(inner, context) => {
                        FnStatementAnalyzer::analyze(scope, inner, context)?
                    }
                    TypeStatementVariant::Contract(inner) => {
                        (ContractStatementAnalyzer::analyze(scope, inner)?, None)
                    }
                };

                self.state.replace(Some(State::Resolved {
                    inner: r#type.clone(),
                    intermediate,
                }));

                Ok(r#type)
            }
            Some(State::Resolved {
                inner,
                intermediate,
            }) => {
                self.state.replace(Some(State::Resolved {
                    inner: inner.clone(),
                    intermediate,
                }));

                Ok(inner)
            }
            None => Err(Error::Scope(ScopeError::ReferenceLoop {
                location: self.location.expect(crate::panic::LOCATION_ALWAYS_EXISTS),
            })),
        }
    }

    pub fn is_resolved(&self) -> bool {
        match self.state.borrow().as_ref() {
            Some(State::Resolved { .. }) => true,
            _ => false,
        }
    }

    pub fn is_contract(&self) -> bool {
        match self.state.borrow().as_ref() {
            Some(State::Unresolved {
                inner: TypeStatementVariant::Contract(_),
                ..
            }) => true,
            Some(State::Resolved {
                inner: TypeElement::Contract(_),
                ..
            }) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state.borrow().as_ref() {
            Some(State::Unresolved { inner, .. }) => write!(f, "{}", inner.identifier().name),
            Some(State::Resolved { inner, .. }) => write!(f, "{}", inner),
            None => match self.location {
                Some(location) => write!(f, "<resolving {}>", location),
                None => write!(f, "<resolving>"),
            },
        }
    }
}
