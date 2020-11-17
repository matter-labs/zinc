//!
//! The semantic analyzer scope type item.
//!

pub mod index;
pub mod state;
pub mod statement;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::generator::statement::Statement as GeneratorStatement;
use crate::semantic::analyzer::statement::contract::Analyzer as ContractStatementAnalyzer;
use crate::semantic::analyzer::statement::r#enum::Analyzer as EnumStatementAnalyzer;
use crate::semantic::analyzer::statement::r#fn::Analyzer as FnStatementAnalyzer;
use crate::semantic::analyzer::statement::r#struct::Analyzer as StructStatementAnalyzer;
use crate::semantic::analyzer::statement::r#type::Analyzer as TypeStatementAnalyzer;
use crate::semantic::element::r#type::Type as TypeElement;
use crate::semantic::error::Error;
use crate::semantic::scope::item::index::INDEX as ITEM_INDEX;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::Scope;

use self::state::State;
use self::statement::Statement as TypeStatementVariant;

///
/// The type item, declared using a `type`, `struct`, `enum`, or another statement.
///
#[derive(Debug, Clone)]
pub struct Type {
    /// The location where the type was declared. `None` for intrinsic types.
    pub location: Option<Location>,
    /// The unique type ID, allocated upon declaration.
    pub item_id: usize,
    /// The definition state, which is either `declared` or `defined`.
    pub state: RefCell<Option<State>>,
}

impl Type {
    ///
    /// Creates a declared type, which must be defined during the second pass or when
    /// the item is referenced for the first time.
    ///
    /// Is used during module items hoisting.
    ///
    /// If the declared type is a contract, its items are hoisted to be defined later.
    ///
    pub fn new_declared(
        location: Option<Location>,
        inner: TypeStatementVariant,
        scope: Rc<RefCell<Scope>>,
    ) -> Result<Self, Error> {
        let item_id = ITEM_INDEX.next(format!("type {}", inner.identifier().name));

        let (inner, scope) = match inner {
            TypeStatementVariant::Contract(statement) => {
                let scope = Scope::new_child(
                    statement.identifier.name.clone(),
                    ScopeType::Contract,
                    scope,
                );
                ContractStatementAnalyzer::declare(scope, statement)
                    .map(|(statement, scope)| (TypeStatementVariant::Contract(statement), scope))?
            }
            TypeStatementVariant::Struct(statement) => {
                let scope = Scope::new_child(
                    statement.identifier.name.clone(),
                    ScopeType::Structure,
                    scope,
                );
                (TypeStatementVariant::Struct(statement), scope)
            }
            TypeStatementVariant::Enum(statement) => {
                let scope = Scope::new_child(
                    statement.identifier.name.clone(),
                    ScopeType::Enumeration,
                    scope,
                );
                (TypeStatementVariant::Enum(statement), scope)
            }
            inner => (inner, scope),
        };

        Ok(Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Declared { inner, scope })),
        })
    }

    ///
    /// Creates a defined type, which is ready to be used from anywhere.
    ///
    /// Is used for items which are not hoisted.
    ///
    pub fn new_defined(
        location: Option<Location>,
        inner: TypeElement,
        is_alias: bool,
        intermediate: Option<GeneratorStatement>,
    ) -> Self {
        let title = format!(
            "{}{}",
            inner.to_string(),
            if is_alias {
                format!(" ({})", Keyword::SelfUppercase.to_string())
            } else {
                "".to_owned()
            }
        );
        let item_id = ITEM_INDEX.next(title);

        Self {
            location,
            item_id,
            state: RefCell::new(Some(State::Defined {
                inner,
                intermediate,
            })),
        }
    }

    ///
    /// Useful method to declare an intrinsic type without a `location` or `intermediate` representation.
    ///
    pub fn new_built_in(inner: TypeElement) -> Self {
        let item_id = ITEM_INDEX.next(inner.to_string());

        Self {
            location: None,
            item_id,
            state: RefCell::new(Some(State::Defined {
                inner,
                intermediate: None,
            })),
        }
    }

    ///
    /// Defines the declared type.
    ///
    /// The method is able to detect reference loops. It happens naturally when the method
    /// is reentered before the item being defined is put back into `variant`, which means that
    /// the item is taken twice during its resolution process.
    ///
    pub fn define(&self) -> Result<TypeElement, Error> {
        let variant = self.state.borrow_mut().take();

        match variant {
            Some(State::Declared { inner, scope }) => {
                let (r#type, intermediate) = match inner {
                    TypeStatementVariant::Type(inner) => {
                        (TypeStatementAnalyzer::define(scope, inner)?, None)
                    }
                    TypeStatementVariant::Struct(inner) => {
                        (StructStatementAnalyzer::define(scope, inner)?, None)
                    }
                    TypeStatementVariant::Enum(inner) => {
                        (EnumStatementAnalyzer::define(scope, inner)?, None)
                    }
                    TypeStatementVariant::Fn(inner) => FnStatementAnalyzer::define(scope, inner)
                        .map(|(r#type, intermediate)| {
                            (r#type, intermediate.map(GeneratorStatement::Fn))
                        })?,
                    TypeStatementVariant::Contract(inner) => ContractStatementAnalyzer::define(
                        scope, inner,
                    )
                    .map(|(r#type, intermediate)| {
                        (r#type, Some(GeneratorStatement::Contract(intermediate)))
                    })?,
                };

                self.state.replace(Some(State::Defined {
                    inner: r#type.clone(),
                    intermediate,
                }));

                Ok(r#type)
            }
            Some(State::Defined {
                inner,
                intermediate,
            }) => {
                self.state.replace(Some(State::Defined {
                    inner: inner.clone(),
                    intermediate,
                }));

                Ok(inner)
            }
            None => Err(Error::ScopeReferenceLoop {
                location: self.location.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
            }),
        }
    }

    ///
    /// Checks whether the type is a contract.
    ///
    pub fn is_contract(&self) -> bool {
        match self.state.borrow().as_ref() {
            Some(State::Declared {
                inner: TypeStatementVariant::Contract(_),
                ..
            }) => true,
            Some(State::Defined {
                inner: TypeElement::Contract(_),
                ..
            }) => true,
            _ => false,
        }
    }

    ///
    /// Extracts the intermediate representation from the element.
    ///
    pub fn get_intermediate(&self) -> Vec<GeneratorStatement> {
        self.state
            .borrow()
            .as_ref()
            .map(|state| state.get_intermediate())
            .unwrap_or_default()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state.borrow().as_ref() {
            Some(State::Declared { inner, .. }) => write!(f, "{}", inner.identifier().name),
            Some(State::Defined { inner, .. }) => write!(f, "{}", inner),
            None => match self.location {
                Some(location) => write!(f, "<resolving {}>", location),
                None => write!(f, "<resolving>"),
            },
        }
    }
}
