//!
//! The type caster error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    FromInvalidType(String, String),
    ToInvalidType(String, String),
    ToLesserBitlength(usize, usize),
}
