//!
//! The lexical comment parser error.
//!

///
/// The lexical comment parser error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The lexeme is not a comment, which means that another parser must be run.
    NotAComment,
    /// The comment has not been terminated, which ends up with an entire file treated as an unterminated comment.
    UnterminatedBlock {
        /// The number of lines in the unterminated comment.
        lines: usize,
        /// The column where the unterminated comment ends.
        column: usize,
    },
}
