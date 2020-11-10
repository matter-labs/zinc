//!
//! The constant array element tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_pushing_invalid_type() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 2] = [1, false];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArrayPushingInvalidType {
        location: Location::test(3, 32),
        expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_index_out_of_range() {
    let input = r#"
fn main() {
    const VALUE: u8 = [1, 2, 3, 4, 5][5];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArrayIndexOutOfRange {
        location: Location::test(3, 39),
        index: BigInt::from(5).to_string(),
        size: 5,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_start_out_of_range() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 2] = [1, 2, 3, 4, 5][-1 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArraySliceStartOutOfRange {
        location: Location::test(3, 45),
        start: BigInt::from(-1).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_out_of_range() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 6] = [1, 2, 3, 4, 5][0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArraySliceEndOutOfRange {
        location: Location::test(3, 44),
        end: BigInt::from(6).to_string(),
        size: 5,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    const ARRAY: [u8; 1] = [1, 2, 3, 4, 5][2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ArraySliceEndLesserThanStart {
            location: Location::test(3, 44),
            start: BigInt::from(2).to_string(),
            end: BigInt::from(1).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
