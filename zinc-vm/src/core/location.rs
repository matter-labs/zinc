use std::fmt;
use std::fmt::{Error, Formatter};

pub struct CodeLocation {
    pub file: Option<String>,
    pub function: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl CodeLocation {
    pub fn new() -> Self {
        Self {
            file: None,
            function: None,
            line: None,
            column: None
        }
    }
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let file = match &self.file {
            Some(file) => file.as_str(),
            None => "<unknown file>",
        };
        write!(f, "{}", file);

        let line = match self.line {
            Some(line) => line.to_string(),
            None => "<unknown line>".into(),
        };
        write!(f, ":{}", line);

        match self.column {
            Some(column) => { write!(f, ":{}", column); },
            None => {},
        };

        match &self.function {
            Some(function) => { write!(f, " (at {})", function); },
            None => {},
        };

        Ok(())
    }
}
