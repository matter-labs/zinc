//!
//! Transpiler output loop.
//!

use crate::Element;

pub struct Output {
    pub r#for: String,
    pub r#while: Option<String>,
    pub end: String,
}

impl Output {
    pub fn output(
        index: String,
        range_start: usize,
        range_end: usize,
        is_inclusive: bool,
        while_condition: Option<Element>,
    ) -> Self {
        let operator = if is_inclusive { "..=" } else { ".." };
        let range = if range_end >= range_start {
            format!("{0}{1}{2}", range_start, operator, range_end)
        } else {
            format!("({0}{1}{2}).rev()", range_end, operator, range_start)
        };

        let r#for = format!("for {0}_index in {1} {{", index, range,);

        let r#while = if let Some(r#while) = while_condition {
            Some(format!(
                r#"if {0}.get_value().expect("Always returns as value") {{"#,
                r#while
            ))
        } else {
            None
        };

        let end = "}".to_owned();

        Self {
            r#for,
            r#while,
            end,
        }
    }
}
