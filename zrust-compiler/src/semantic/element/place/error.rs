//!
//! The semantic analyzer element place error.
//!

use failure::Fail;

use crate::semantic::Value;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "indexing requires an integer constant, but got '{}'", _0)]
    IndexingExpectedIntegerConstant(Value),
    #[fail(
        display = "tuple access requires an integer constant, but got '{}'",
        _0
    )]
    TupleAccessExpectedIntegerConstant(Value),
}
