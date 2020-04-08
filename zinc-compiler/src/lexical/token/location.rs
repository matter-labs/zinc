//!
//! The lexical token location.
//!

use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Location {
    pub file_index: Option<usize>,
    pub line: usize,
    pub column: usize,
}

impl Location {
    ///
    /// Creates a location without a file identifier.
    /// Used mostly for testing purposes.
    ///
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            file_index: None,
            line,
            column,
        }
    }

    ///
    /// Creates a location with a file identifier.
    /// The file identifier can be used to get its path from the global type index.
    ///
    pub fn new_beginning(file_index: Option<usize>) -> Self {
        Self {
            file_index,
            line: 1,
            column: 1,
        }
    }

    ///
    /// Creates a location by shifting the original down by `lines` and
    /// setting the column to `column`.
    ///
    pub fn shifted_down(&self, lines: usize, column: usize) -> Self {
        Self {
            file_index: self.file_index,
            line: self.line + lines,
            column,
        }
    }

    ///
    /// Creates a location by shifting the original right by `columns`.
    ///
    pub fn shifted_right(&self, columns: usize) -> Self {
        Self {
            file_index: self.file_index,
            line: self.line,
            column: self.column + columns,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.line, self.column) {
            (0, 0) => write!(f, "<unavailable>"),
            (line, column) => match self.file_index {
                Some(file_index) => write!(
                    f,
                    "{}:{}:{}",
                    crate::file::INDEX
                        .read()
                        .expect(crate::PANIC_MUTEX_SYNC)
                        .get(file_index)
                        .expect(crate::PANIC_FILE_INDEX)
                        .to_string_lossy(),
                    line,
                    column,
                ),
                None => write!(f, "{}:{}", line, column),
            },
        }
    }
}
