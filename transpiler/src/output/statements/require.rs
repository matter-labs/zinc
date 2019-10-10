//!
//! Transpiler output require statement.
//!

pub struct Output {
    pub expression: String,
    pub annotation: String,
}

impl Output {
    pub fn new(expression: String, annotation: String) -> Self {
        Self {
            expression,
            annotation,
        }
    }
}

impl Into<String> for Output {
    fn into(self) -> String {
        format!(
            r#"r1cs::require(system.namespace(|| "{1}"), &{0}, "{1}");"#,
            self.expression, self.annotation
        )
    }
}
