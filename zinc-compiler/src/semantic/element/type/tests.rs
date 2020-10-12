//!
//! The type tests.
//!

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::error::Error;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_alias_does_not_point_to_type() {
    let input = r#"
fn main() {
    let unknown = 0;
    let result = 42 as unknown;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::AliasDoesNotPointToType {
            location: Location::test(4, 24),
            found: Element::Path(Path::new(
                Location::test(4, 24),
                Identifier::new(Location::test(4, 24), "unknown".to_owned()),
            ))
            .to_string(),
        },
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
