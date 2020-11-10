//!
//! The semantic analyzer library function element.
//!

#[cfg(test)]
mod tests;

pub mod transfer;

use std::fmt;

use zinc_build::LibraryFunctionIdentifier;
use zinc_lexical::Location;

use crate::semantic::element::argument_list::ArgumentList;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error;

use self::transfer::Function as TransferFunction;

///
/// The semantic analyzer standard library function element.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `zksync::transfer` function variant.
    Transfer(TransferFunction),
}

impl Function {
    ///
    /// Calls the function with the `argument_list`, validating the call.
    ///
    pub fn call(self, location: Location, argument_list: ArgumentList) -> Result<Type, Error> {
        match self {
            Self::Transfer(inner) => inner.call(location, argument_list),
        }
    }

    ///
    /// Returns the function identifier, which is known at compile time.
    ///
    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Transfer(inner) => inner.identifier,
        }
    }

    ///
    /// The unique standard library function identifier.
    ///
    pub fn library_identifier(&self) -> LibraryFunctionIdentifier {
        match self {
            Self::Transfer(inner) => inner.library_identifier,
        }
    }

    ///
    /// Whether the function must be called from mutable context.
    ///
    pub fn is_mutable(&self) -> bool {
        match self {
            Self::Transfer(_) => true,
        }
    }

    ///
    /// Sets the function call location in the code.
    ///
    pub fn set_location(&mut self, location: Location) {
        match self {
            Self::Transfer(inner) => inner.location = Some(location),
        }
    }

    ///
    /// Returns the location of the function call.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Transfer(inner) => inner.location,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transfer(inner) => write!(f, "{}", inner),
        }
    }
}
