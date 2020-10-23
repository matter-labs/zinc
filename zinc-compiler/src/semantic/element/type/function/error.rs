//!
//! The semantic analyzer function error.
//!

use zinc_lexical::Location;

use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;

///
/// The semantic analyzer function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The actual arguments number does not match the formal arguments number.
    ArgumentCount {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The expected number of arguments.
        expected: usize,
        /// The actual number of arguments.
        found: usize,
        /// The reference to the function argument list location in the function prototype.
        reference: Option<Location>,
    },
    /// The argument type does not match the expected one.
    ArgumentType {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The name of the argument, whose actual type is invalid.
        name: String,
        /// The position of the invalid argument.
        position: usize,
        /// The stringified expected type.
        expected: String,
        /// The actual invalid type.
        found: String,
    },
    /// The argument must be a constant, but it is not.
    ArgumentConstantness {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The name of the non-constant argument.
        name: String,
        /// The position of the non-constant argument.
        position: usize,
        /// The stringified non-constant argument.
        found: String,
    },
    /// The argument cannot be treated as value.
    ArgumentNotEvaluable {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The position of the invalid argument.
        position: usize,
        /// The stringified invalid argument.
        found: String,
    },
    /// The function returns a value, whose type does not match the one in the function prototype.
    ReturnType {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
        /// The stringified type, which is expected.
        expected: String,
        /// The stringified invalid type.
        found: String,
        /// The reference to the return type location in the function prototype.
        reference: Location,
    },
    /// Calling a non-callable object, that is, not a function.
    NonCallable {
        /// The error location data.
        location: Location,
        /// The non-callable item identifier.
        name: String,
    },
    /// The `self` argument of a method must be first in argument list, but it is not.
    FunctionMethodSelfNotFirst {
        /// The function location.
        location: Location,
        /// The function identifier.
        function: String,
        /// The invalid position of the `self` argument.
        position: usize,
        /// The invalid argument location.
        reference: Location,
    },
    /// A mutable method was called with an immutable instance.
    CallingMutableFromImmutable {
        /// The function location.
        location: Location,
        /// The function identifier.
        function: String,
    },

    /// The intrinsic function error. See the inner element description.
    Intrinsic(IntrinsicFunctionError),
    /// The unit test function error. See the inner element description.
    Test(TestFunctionError),
}
