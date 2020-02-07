//!
//! The syntax error.
//!

use failure::Fail;

use crate::lexical::Lexeme;
use crate::lexical::Location;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{}: expected either of: {:?} (got '{}')", _0, _1, _2)]
    Expected(Location, Vec<&'static str>, Lexeme),
}
