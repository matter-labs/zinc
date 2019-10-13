//!
//! Transpiler output attributes.
//!

pub struct Output {}

impl Output {
    pub fn output() -> Vec<String> {
        vec![
            "#![allow(unused_imports)]".to_owned(),
            "#![allow(unused_variables)]".to_owned(),
            "#![allow(unused_assignments)]".to_owned(),
            "#![allow(clippy::let_and_return)]".to_owned(),
            String::new(),
        ]
    }
}
