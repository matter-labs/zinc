//!
//! Transpiler output let statement.
//!

use parser::Identifier;
use parser::Type;

use crate::Element;
use crate::TypeOutput;

pub struct Output {}

impl Output {
    pub fn output(
        is_mutable: bool,
        identifier: Identifier,
        r#type: Option<Type>,
        expression: Element,
    ) -> String {
        format!(
            "let{0} {1}{2} = {3};",
            if is_mutable { " mut" } else { "" },
            identifier,
            if let Some(r#type) = r#type.map(|r#type| TypeOutput::output(r#type.variant)) {
                format!(": {}", r#type)
            } else {
                "".to_owned()
            },
            match expression {
                Element::Permanent(element) => format!("{}.clone()", element),
                element => element.to_string(),
            }
        )
    }
}
