//!
//! The generator intermediate language.
//!

pub mod expression;
pub mod statement;
pub mod r#type;

use self::statement::Statement;

pub static PANIC_VALIDATED_DURING_SEMANTIC_ANALYSIS: &str =
    "Validated during the semantic analysis";

#[derive(Default)]
pub struct Representation {
    pub statements: Vec<Statement>,
}

impl Representation {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}
