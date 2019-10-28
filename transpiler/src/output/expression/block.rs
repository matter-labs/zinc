//!
//! Transpiler output block.
//!

pub struct Output {
    pub start: String,
    pub end: String,
}

impl Output {
    pub fn output(identifier: String) -> Self {
        let start = format!("let {0} = {{", identifier);
        let end = "};".to_owned();
        Self { start, end }
    }
}
