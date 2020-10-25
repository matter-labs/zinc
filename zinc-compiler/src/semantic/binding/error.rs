//!
//! The variable binder error.
//!

use zinc_lexical::Location;

///
/// The type caster error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The binding list expects a tuple expression.
    ExpectedTuple {
        /// The invalid pattern location.
        location: Location,
        /// The expected tuple length.
        expected: usize,
        /// The found type.
        found: String,
    },
    /// The `self` argument of a method must be first in argument list, but it is not.
    FunctionMethodSelfNotFirst {
        /// The invalid argument location.
        location: Location,
        /// The binding identifier.
        name: String,
        /// The invalid argument position.
        position: usize,
    },
    /// Tuple function argument destructuring is not implemented yet.
    FunctionArgumentDestructuringUnavailable {
        /// The error location.
        location: Location,
    },
}
