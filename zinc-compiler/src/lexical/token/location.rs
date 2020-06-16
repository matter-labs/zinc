//!
//! The lexical token location.
//!

use std::fmt;

use crate::source::file::index::INDEX as FILE_INDEX;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Location {
    pub file_index: usize,
    pub line: usize,
    pub column: usize,
}

impl Location {
    ///
    /// Creates a location without a default file identifier.
    /// Used mostly for testing purposes.
    ///
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            file_index: 0,
            line,
            column,
        }
    }

    ///
    /// Creates a location with a file identifier.
    /// Used mostly for testing purposes.
    ///
    pub fn new_with_file_id(line: usize, column: usize, file_index: usize) -> Self {
        Self {
            file_index,
            line,
            column,
        }
    }

    ///
    /// Creates a location with a file identifier.
    /// The file identifier can be used to get its contents from the global index.
    ///
    pub fn new_beginning(file_index: Option<usize>) -> Self {
        Self {
            file_index: file_index.unwrap_or_default(),
            line: 1,
            column: 1,
        }
    }

    ///
    /// Creates a location by shifting the original one down by `lines` and
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
    /// Creates a location by shifting the original one rightward by `columns`.
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (0, 0) => write!(f, "<unavailable>"),
            (line, column) => write!(
                f,
                "{}:{}:{}",
                FILE_INDEX.get_path(self.file_index).to_string_lossy(),
                line,
                column,
            ),
        }
    }
}
