//!
//! The semantic analyzer function type element.
//!

pub mod assert;
pub mod debug;
pub mod standard;
pub mod user;

use std::fmt;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::Type;

use self::assert::AssertInstructionFunction;
use self::debug::DebugInstructionFunction;
use self::standard::Function as StandardFunction;
use self::user::Function as UserFunction;

#[derive(Debug, Clone)]
pub enum Function {
    DebugInstruction(DebugInstructionFunction),
    AssertInstruction(AssertInstructionFunction),
    StandardLibrary(StandardFunction),
    UserDefined(UserFunction),
}

impl Function {
    pub fn new_dbg() -> Self {
        Self::DebugInstruction(DebugInstructionFunction::new())
    }

    pub fn new_assert() -> Self {
        Self::AssertInstruction(AssertInstructionFunction::new())
    }

    pub fn new_std(identifier: BuiltinIdentifier) -> Self {
        Self::StandardLibrary(StandardFunction::new(identifier))
    }

    pub fn new_user_defined(
        identifier: String,
        arguments: Vec<(String, Type)>,
        return_type: Type,
    ) -> Self {
        Self::UserDefined(UserFunction::new(identifier, arguments, return_type))
    }

    pub fn identifier(&self) -> String {
        match self {
            Function::DebugInstruction(inner) => inner.identifier.to_owned(),
            Function::AssertInstruction(inner) => inner.identifier.to_owned(),
            Function::StandardLibrary(inner) => inner.identifier().to_owned(),
            Function::UserDefined(inner) => inner.identifier.to_owned(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DebugInstruction(inner) => write!(f, "{}", inner),
            Self::AssertInstruction(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "{}", inner),
            Self::UserDefined(inner) => write!(f, "{}", inner),
        }
    }
}
