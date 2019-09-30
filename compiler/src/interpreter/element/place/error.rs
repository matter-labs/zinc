//!
//! The interpreter place error.
//!

use failure::Fail;

use crate::interpreter::Value;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "indexing requires an integer constant, but got: [{}]", _0)]
    IndexingExpectedIntegerConstant(Value),
}
