//!
//! The semantic analyzer function element.
//!

mod tests;

pub mod builtin;
pub mod error;
pub mod stdlib;
pub mod user;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::Type;

use self::builtin::Function as BuiltInFunction;
use self::stdlib::Function as StandardLibraryFunction;
use self::user::Function as UserFunction;

///
/// Describes a function, which is a special type.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// `dbg!` and `assert!`, which must be called with the `!` specifier. These correspond to
    /// some special VM instructions.
    BuiltInFunction(BuiltInFunction),
    /// These functions are declared in a virtual built-in scope and implemented in the VM
    /// as built-in function calls.
    StandardLibrary(StandardLibraryFunction),
    /// Ordinar functions declared anywhere within a project. There is a special `main` function,
    /// which is also declared by user, but serves as the circuit entry point.
    UserDefined(UserFunction),
}

impl Function {
    pub fn new_dbg() -> Self {
        Self::BuiltInFunction(BuiltInFunction::new_debug())
    }

    pub fn new_assert() -> Self {
        Self::BuiltInFunction(BuiltInFunction::new_assert())
    }

    pub fn new_std(identifier: BuiltinIdentifier) -> Self {
        Self::StandardLibrary(StandardLibraryFunction::new(identifier))
    }

    pub fn new_user_defined(
        identifier: String,
        unique_id: usize,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self::UserDefined(UserFunction::new(
            identifier,
            unique_id,
            arguments,
            return_type,
        ))
    }

    pub fn identifier(&self) -> String {
        match self {
            Function::BuiltInFunction(inner) => inner.identifier().to_owned(),
            Function::StandardLibrary(inner) => inner.identifier().to_owned(),
            Function::UserDefined(inner) => inner.identifier().to_owned(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BuiltInFunction(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "{}", inner),
            Self::UserDefined(inner) => write!(f, "{}", inner),
        }
    }
}
