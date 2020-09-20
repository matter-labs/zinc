//!
//! The lexical token location.
//!

use std::fmt;

use crate::source::file::index::INDEX as FILE_INDEX;

///
/// The token location in the source code file.
///
#[derive(Debug, Default, Clone, Copy)]
pub struct Location {
    /// The line number, starting from 1.
    pub line: usize,
    /// The column number, starting from 1.
    pub column: usize,
    /// The file unique identifier, stored in the file index.
    pub file: usize,
}

impl Location {
    ///
    /// Creates a location with a file identifier.
    /// The file identifier can be used to get its contents from the global index.
    ///
    pub fn new(file: usize) -> Self {
        Self {
            line: 1,
            column: 1,
            file,
        }
    }

    ///
    /// Creates a location by shifting the original one down by `lines` and
    /// setting the column to `column`.
    ///
    pub fn shifted_down(&self, lines: usize, column: usize) -> Self {
        Self {
            line: self.line + lines,
            column,
            file: self.file,
        }
    }

    ///
    /// Creates a location by shifting the original one rightward by `columns`.
    ///
    pub fn shifted_right(&self, columns: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + columns,
            file: self.file,
        }
    }

    ///
    /// Creates a location without a default file identifier.
    /// Used for testing purposes.
    ///
    pub fn test(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
            file: FILE_INDEX.current(),
        }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (0, 0) => write!(f, "<unavailable>"),
            (line, column) => write!(
                f,
                "{}:{}:{}",
                FILE_INDEX.get_path(self.file).to_string_lossy(),
                line,
                column,
            ),
        }
    }
}
