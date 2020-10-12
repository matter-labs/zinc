//!
//! The lexical token location.
//!

use std::fmt;

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
    /// Creates a location for testing purposes.
    ///
    /// If the `file_index` feature is enabled, fetches the current file index
    /// from the global storage.
    ///
    pub fn test(line: usize, column: usize) -> Self {
        let file = 0;

        #[cfg(file_index)]
        let file = zinc_utils::FILE_INDEX.current();

        Self { line, column, file }
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
            (line, column) => {
                let file = "<unavailable>".to_owned();

                #[cfg(file_index)]
                let file = zinc_utils::FILE_INDEX.get_path(self.file).to_string_lossy();

                write!(f, "{}:{}:{}", file, line, column,)
            }
        }
    }
}
