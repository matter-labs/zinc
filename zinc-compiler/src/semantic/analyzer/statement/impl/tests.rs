//!
//! The `impl` statement tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::error::Error as StatementError;

#[test]
fn error_expected_namespace() {
    let input = r#"
type X = field;

impl X {
    fn impossible() {}
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(StatementError::Impl(ImplStatementError::ExpectedNamespace {
        location: Location::new(4, 6),
            found: Type::field().to_string(),
        },
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
