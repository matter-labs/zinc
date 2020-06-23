//!
//! The lexical token comment lexeme.
//!

use std::fmt;

///
/// The source code comment, which is dropped during the lexical analysis.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Comment {
    /// The line comment like `// text`
    Line {
        /// The inner comment contents.
        inner: String,
    },
    /// The block comment like `/* text */`
    Block {
        /// The inner comment contents.
        inner: String,
    },
}

impl Comment {
    ///
    /// Creates a single-line comment.
    ///
    pub fn new_line(inner: String) -> Self {
        Self::Line { inner }
    }

    ///
    /// Creates a block comment.
    ///
    pub fn new_block(inner: String) -> Self {
        Self::Block { inner }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line { inner } => write!(f, "{}", inner),
            Self::Block { inner } => write!(f, "{}", inner),
        }
    }
}
