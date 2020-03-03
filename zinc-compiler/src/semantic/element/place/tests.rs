//!
//! The place element tests.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn error_element_place_field_1st_expected_structure() {
    let input = r#"
fn main() {
    let tuple = (true, true, false);
    let value = tuple.first;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedStructure(
            Type::tuple(vec![Type::boolean(); 3]).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_field_1st_expected_tuple() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let data = Data {
        a: 0,
    };
    let value = data.0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(10, 21),
        ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedTuple(
            Type::structure(
                "Data".to_owned(),
                1,
                vec![(
                    "a".to_owned(),
                    Type::integer_unsigned(crate::BITLENGTH_BYTE),
                )],
                None,
            )
            .to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_field_does_not_exist_in_structure() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let data = Data {
        a: 0,
    };
    let value = data.b;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(10, 21),
        ElementError::Place(PlaceError::StructureFieldDoesNotExist(
            "b".to_owned(),
            "Data".to_owned(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_field_does_not_exist_in_tuple() {
    let input = r#"
fn main() {
    let tuple = (1, 2, 3);
    let result = tuple.5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 23),
        ElementError::Place(PlaceError::TupleFieldDoesNotExist(
            5,
            Type::tuple(vec![Type::integer_unsigned(crate::BITLENGTH_BYTE); 3]).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_index_1st_expected_array() {
    let input = r#"
fn main() {
    let tuple = (true, false, true);
    let value = tuple[1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::OperatorIndexFirstOperandExpectedArray(
            Type::tuple(vec![Type::boolean(); 3]).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_index_2nd_expected_integer() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let value = array[true];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(
            PlaceError::OperatorIndexSecondOperandExpectedIntegerOrRange(
                Constant::Boolean(true).to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_index_slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::ArraySliceEndLesserThanStart(
            "1".to_owned(),
            "2".to_owned(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_index_slice_end_out_of_range() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::ArraySliceEndOutOfRange("6".to_owned(), 5)),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_place_index_slice_start_out_of_range() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[-1 .. 1 as i8];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::ArraySliceStartOutOfRange("-1".to_owned())),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
