//!
//! The virtual machine code location.
//!

use std::fmt;

#[derive(Clone)]
pub struct Location {
    pub file: Option<String>,
    pub function: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            function: None,
            line: None,
            column: None,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = match &self.file {
            Some(file) => file.as_str(),
            None => "<unknown file>",
        };
        write!(f, "{}", file)?;

        let line = match self.line {
            Some(line) => line.to_string(),
            None => "<unknown line>".into(),
        };
        write!(f, ":{}", line)?;

        if let Some(column) = self.column {
            write!(f, ":{}", column)?;
        }

        if let Some(function) = &self.function {
            write!(f, " (at {})", function)?;
        }

        Ok(())
    }
}
