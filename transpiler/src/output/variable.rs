//!
//! Transpiler output variable.
//!

pub struct Output {
    pub identifier: String,
    pub is_temporary: bool,
}

impl Output {
    pub fn new(identifier: String, is_temporary: bool) -> Self {
        Self {
            identifier,
            is_temporary,
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        self.identifier
    }
}
