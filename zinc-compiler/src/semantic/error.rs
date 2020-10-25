//!
//! The semantic analyzer error.
//!

use zinc_lexical::Location;

use crate::semantic::analyzer::attribute::error::Error as AttributeError;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::binding::error::Error as BindingError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::scope::error::Error as ScopeError;

///
/// The semantic analyzer error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The semantic element error.
    Element(ElementError),
    /// The scope error.
    Scope(ScopeError),
    /// The expression analysis error.
    Expression(ExpressionError),
    /// The statement analysis error.
    Statement(StatementError),
    /// The attribute analysis error.
    Attribute(AttributeError),
    /// The variable binding error.
    Binding(BindingError),

    /// The application does not have an entry point function.
    EntryPointMissing,
    /// The application has both the `main` function and contract.
    EntryPointAmbiguous {
        /// The location where the `main` function is declared.
        main: Location,
        /// The location where the contract is declared.
        contract: Location,
    },
    /// The application entry function cannot be constant.
    EntryPointConstant {
        /// The location where the constant `main` function is declared.
        location: Location,
    },
    /// The application entry `main` function is declared outside the application entry module.
    FunctionMainBeyondEntry {
        /// The location where the `main` function is declared.
        location: Location,
    },
    /// The application contract is declared outside the application entry module.
    ContractBeyondEntry {
        /// The location where the contract is declared.
        location: Location,
    },
    /// The source code file for module `name` cannot be found.
    ModuleFileNotFound {
        /// The location where the module is declared.
        location: Location,
        /// The module name, source code for which is absent.
        name: String,
    },
}

impl From<ScopeError> for Error {
    fn from(inner: ScopeError) -> Self {
        Self::Scope(inner)
    }
}
