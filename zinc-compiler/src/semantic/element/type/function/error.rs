//!
//! The semantic analyzer function error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::function::builtin::error::Error as BuiltInFunctionTypeError;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionTypeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    ArgumentCount {
        function: String,
        expected: usize,
        found: usize,
    },
    ArgumentType {
        function: String,
        name: String,
        position: usize,
        expected: String,
        found: String,
    },
    ArgumentConstantness {
        function: String,
        name: String,
        position: usize,
        found: String,
    },
    ArgumentNotEvaluable {
        function: String,
        position: usize,
        found: String,
    },
    ReturnType {
        function: String,
        expected: String,
        found: String,
        reference: Location,
    },
    NonCallable {
        name: String,
    },
    FunctionMethodSelfNotFirst {
        function: String,
        position: usize,
        reference: Location,
    },

    BuiltIn(BuiltInFunctionTypeError),
    StandardLibrary(StandardLibraryFunctionTypeError),
}

impl Error {
    pub fn argument_count(function: String, expected: usize, found: usize) -> Self {
        Self::ArgumentCount {
            function,
            expected,
            found,
        }
    }

    pub fn argument_type(
        function: String,
        name: String,
        position: usize,
        expected: String,
        found: String,
    ) -> Self {
        Self::ArgumentType {
            function,
            name,
            position,
            expected,
            found,
        }
    }

    pub fn argument_constantness(
        function: String,
        name: String,
        position: usize,
        found: String,
    ) -> Self {
        Self::ArgumentConstantness {
            function,
            name,
            position,
            found,
        }
    }

    pub fn argument_not_evaluable(function: String, position: usize, found: String) -> Self {
        Self::ArgumentNotEvaluable {
            function,
            position,
            found,
        }
    }

    pub fn return_type(
        function: String,
        expected: String,
        found: String,
        reference: Location,
    ) -> Self {
        Self::ReturnType {
            function,
            expected,
            found,
            reference,
        }
    }

    pub fn non_callable(name: String) -> Self {
        Self::NonCallable { name }
    }

    pub fn function_method_self_not_first(
        function: String,
        position: usize,
        reference: Location,
    ) -> Self {
        Self::FunctionMethodSelfNotFirst {
            function,
            position,
            reference,
        }
    }
}
