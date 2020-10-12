//!
//! The semantic analyzer function element.
//!

pub mod constant;
pub mod error;
pub mod intrinsic;
pub mod runtime;
pub mod test;

use std::fmt;

use zinc_build::LibraryFunctionIdentifier;
use zinc_lexical::Location;
use zinc_syntax::BlockExpression;

use crate::semantic::element::r#type::Type;

use self::constant::Function as ConstantFunction;
use self::intrinsic::Function as IntrinsicFunction;
use self::runtime::Function as RuntimeFunction;
use self::test::Function as TestFunction;

///
/// Describes a function, which is a special type.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `dbg!` function, which must be called with the `!` specifier, and the `require` function.
    /// These correspond to some special VM instructions.
    /// Also, standard library and zkSync library functions, which are declared in a virtual intrinsic
    /// scope and implemented in the VM as intrinsic function calls.
    Intrinsic(IntrinsicFunction),
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
    ///
    /// A shortcut constructor.
    ///
    pub fn new_dbg() -> Self {
        Self::Intrinsic(IntrinsicFunction::new_debug())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_require() -> Self {
        Self::Intrinsic(IntrinsicFunction::new_require())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_library(identifier: LibraryFunctionIdentifier) -> Self {
        Self::Intrinsic(IntrinsicFunction::new_library(identifier))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_runtime(
        location: Location,
        identifier: String,
        type_id: usize,
        arguments: Vec<(String, bool, Type)>,
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

    ///
    /// A shortcut constructor.
    ///
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

    ///
    /// A shortcut constructor.
    ///
    pub fn new_test(location: Location, identifier: String, type_id: usize) -> Self {
        Self::Test(TestFunction::new(location, identifier, type_id))
    }

    ///
    /// Returns the function identifier.
    ///
    pub fn identifier(&self) -> String {
        match self {
            Self::Intrinsic(inner) => inner.identifier().to_owned(),
            Self::Runtime(inner) => inner.identifier.to_owned(),
            Self::Constant(inner) => inner.identifier.to_owned(),
            Self::Test(inner) => inner.identifier.to_owned(),
        }
    }

    ///
    /// Sets the location for the function element.
    ///
    pub fn set_location(&mut self, value: Location) {
        match self {
            Self::Intrinsic(inner) => inner.set_location(value),
            Self::Runtime(inner) => inner.location = value,
            Self::Constant(inner) => inner.location = value,
            Self::Test(inner) => inner.location = value,
        }
    }

    ///
    /// Returns the location of the function element.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Intrinsic(inner) => inner.location(),
            Self::Runtime(inner) => Some(inner.location),
            Self::Constant(inner) => Some(inner.location),
            Self::Test(inner) => Some(inner.location),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Intrinsic(inner) => write!(f, "{}", inner),
            Self::Runtime(inner) => write!(f, "{}", inner),
            Self::Constant(inner) => write!(f, "{}", inner),
            Self::Test(inner) => write!(f, "{}", inner),
        }
    }
}
