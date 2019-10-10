//!
//! Transpiler output debug statement.
//!

pub struct Output {
    pub expression: String,
}

impl Output {
    pub fn new(expression: String) -> Self {
        Self { expression }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(r#"dbg!(&{0}.get_value());"#, self.expression)
    }
}
