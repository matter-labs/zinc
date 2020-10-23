//!
//! The lexical comment parser output.
//!

use crate::token::lexeme::comment::Comment;

///
/// The lexical comment parser output.
///
#[derive(Debug, PartialEq)]
pub struct Output {
    /// The number of characters in the comment.
    pub size: usize,
    /// The numbers of lines in the comment.
    pub lines: usize,
    /// The column where the comment ends.
    pub column: usize,
    /// The comment data.
    pub comment: Comment,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(size: usize, lines: usize, column: usize, comment: Comment) -> Self {
        Self {
            size,
            lines,
            column,
            comment,
        }
    }
}
