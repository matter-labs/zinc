//!
//! Transpiler output while loop.
//!

use crate::element::Element;

pub struct Output {
    pub start: String,
    pub end: String,
}

impl Output {
    pub fn output(while_condition: Element) -> Self {
        let start = format!(
            r#"if {0}.get_value().expect("Always returns a value") {{"#,
            while_condition
        );
        let end = "} else { break; }".to_owned();

        Self { start, end }
    }
}
