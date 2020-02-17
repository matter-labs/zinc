//!
//! The lexical token location.
//!

use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Location {
    pub file: Option<usize>,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            file: None,
            line,
            column,
        }
    }

    pub fn new_beginning(file: Option<usize>) -> Self {
        Self {
            file,
            line: 1,
            column: 1,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.line, self.column) {
            (0, 0) => write!(f, "<unavailable>"),
            (line, column) => match self.file {
                Some(file) => write!(
                    f,
                    "{}:{}:{}",
                    crate::FILE_INDEX
                        .read()
                        .expect(crate::PANIC_MUTEX_SYNC)
                        .get(&file)
                        .expect(crate::PANIC_FILE_INDEX)
                        .to_string_lossy(),
                    line,
                    column
                ),
                None => write!(f, "{}:{}", line, column),
            },
        }
    }
}
