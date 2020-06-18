//!
//! The semantic analyzer function element.
//!

pub mod builtin;
pub mod constant;
pub mod error;
pub mod runtime;
pub mod stdlib;
pub mod test;

use std::fmt;

use zinc_bytecode::BuiltinIdentifier;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::syntax::tree::expression::block::Expression as BlockExpression;

use self::builtin::Function as BuiltInFunction;
use self::constant::Function as ConstantFunction;
use self::runtime::Function as RuntimeFunction;
use self::stdlib::Function as StandardLibraryFunction;
use self::test::Function as TestFunction;

///
/// Describes a function, which is a special type.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// `dbg!` and `assert!`, which must be called with the `!` specifier. These correspond to
    /// some special VM instructions.
    BuiltIn(BuiltInFunction),
    /// These functions are declared in a virtual built-in scope and implemented in the VM
    /// as built-in function calls.
    StandardLibrary(StandardLibraryFunction),
    /// Runtime functions declared anywhere within a project. There is a special `main` function,
    /// which is also declared by user, but serves as the circuit entry point.
    Runtime(RuntimeFunction),
    /// Constant functions declared anywhere within a project. There are executed at compile-time
    /// only and do not produce the intermediate representation.
    Constant(ConstantFunction),
    /// Unit test functions. They produce the intermediate representation and are run as separate
    /// entry points in the special test mode.
    Test(TestFunction),
}

impl Function {
    pub fn new_dbg() -> Self {
        Self::BuiltIn(BuiltInFunction::new_debug())
    }

    pub fn new_assert() -> Self {
        Self::BuiltIn(BuiltInFunction::new_assert())
    }

    pub fn new_std(identifier: BuiltinIdentifier) -> Self {
        Self::StandardLibrary(StandardLibraryFunction::new(identifier))
    }

    pub fn new_runtime(
        location: Location,
        identifier: String,
        type_id: usize,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self::Runtime(RuntimeFunction::new(
            location,
            identifier,
            type_id,
            arguments,
            return_type,
        ))
    }

    pub fn new_constant(
        location: Location,
        identifier: String,
        type_id: usize,
        arguments: Vec<(String, Type)>,
        return_type: Type,
        body: BlockExpression,
    ) -> Self {
        Self::Constant(ConstantFunction::new(
            location,
            identifier,
            type_id,
            arguments,
            return_type,
            body,
        ))
    }

    pub fn new_test(location: Location, identifier: String, type_id: usize) -> Self {
        Self::Test(TestFunction::new(location, identifier, type_id))
    }

    pub fn identifier(&self) -> String {
        match self {
            Self::BuiltIn(inner) => inner.identifier().to_owned(),
            Self::StandardLibrary(inner) => inner.identifier().to_owned(),
            Self::Runtime(inner) => inner.identifier.to_owned(),
            Self::Constant(inner) => inner.identifier.to_owned(),
            Self::Test(inner) => inner.identifier.to_owned(),
        }
    }

    pub fn set_location(&mut self, value: Location) {
        match self {
            Self::BuiltIn(inner) => inner.set_location(value),
            Self::StandardLibrary(inner) => inner.set_location(value),
            Self::Runtime(inner) => inner.location = value,
            Self::Constant(inner) => inner.location = value,
            Self::Test(inner) => inner.location = value,
        }
    }

    pub fn location(&self) -> Option<Location> {
        match self {
            Self::BuiltIn(inner) => inner.location(),
            Self::StandardLibrary(inner) => inner.location(),
            Self::Runtime(inner) => Some(inner.location),
            Self::Constant(inner) => Some(inner.location),
            Self::Test(inner) => Some(inner.location),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuiltIn(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "std::{}", inner),
            Self::Runtime(inner) => write!(f, "{}", inner),
            Self::Constant(inner) => write!(f, "{}", inner),
            Self::Test(inner) => write!(f, "{}", inner),
        }
    }
}
