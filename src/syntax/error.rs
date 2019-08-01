//!
//! The syntax analyzer errors.
//!

use failure::Fail;

use crate::syntax::State;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Expected either of: {:?} (got '{}')", _0, _1)]
    Expected(Vec<&'static str>, String),
    #[fail(display = "Unexpected end at state {:?}", _0)]
    UnexpectedEnd(State),
}
