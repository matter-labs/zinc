//!
//! The place element tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::r#type::Type as ScopeType;
use crate::semantic::scope::Scope;

#[test]
fn ok_mutating_simple_variable() {
    let input = r#"
fn main() {
    let mut result = 42;
    result = 64;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_array() {
    let input = r#"
fn main() {
    let mut result = [1, 2, 3, 4, 5];
    result = [6, 7, 8, 9, 10];
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_array_element() {
    let input = r#"
fn main() {
    let mut result = [1, 2, 3, 4, 5];
    result[3] = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_tuple() {
    let input = r#"
fn main() {
    let mut result = (1, 2, 3, 4, 5);
    result = (6, 7, 8, 9, 10);
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_tuple_element() {
    let input = r#"
fn main() {
    let mut result = (1, 2, 3, 4, 5);
    result.3 = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_structure() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {
    let mut result = Data {
        a: 1,
        b: 2,
        c: 3,
    };

    result = Data {
        a: 10,
        b: 20,
        c: 30,
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_structure_field() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {
    let mut result = Data {
        a: 1,
        b: 2,
        c: 3,
    };

    result.b = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_complex() {
    let input = r#"
struct Data {
    a: (u8, [u8; 4]),
}

fn main() {
    let mut result = Data {
        a: (1, [2; 4]),
    };

    result = Data {
        a: (42, [10; 4]),
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_mutating_complex_element() {
    let input = r#"
struct Data {
    a: (u8, [u8; 4]),
}

fn main() {
    let mut result = Data {
        a: (1, [2; 4]),
    };

    result.a.1[1] = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_mutating_immutable_memory() {
    let input = r#"
fn main() {
    let result = 42;
    result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingImmutableMemory {
        location: Location::test(4, 5),
        name: "result".to_string(),
        reference: Some(Location::test(3, 9)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_mutating_immutable_contract_field_address() {
    let input = r#"
contract Test {
    pub fn mutator(mut self) {
        self.address = 42 as u160;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MutatingImmutableContractField {
            location: Location::test(4, 9),
            name: zinc_const::contract::FIELD_NAME_ADDRESS.to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_mutating_with_different_type() {
    let input = r#"
fn main() {
    let mut result = 42;
    result = false;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MutatingWithDifferentType {
        location: Location::test(4, 5),
        expected: Type::boolean(None).to_string(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_1st_operand_expected_array() {
    let input = r#"
fn main() {
    let tuple = (true, false, true);
    let value = tuple[1];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexFirstOperandExpectedArray {
            location: Location::test(4, 17),
            found: Type::tuple(Some(Location::test(4, 17)), vec![Type::boolean(None); 3])
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_index_2nd_operand_expected_integer_or_range() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let value = array[true];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorIndexSecondOperandExpectedIntegerOrRange {
            location: Location::test(4, 23),
            found: Element::Constant(Constant::Boolean(BooleanConstant::new(
                Location::test(4, 23),
                true,
            )))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_field_1st_operand_expected_tuple() {
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

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDotFirstOperandExpectedTuple {
            location: Location::test(10, 17),
            found: Type::structure(
                Some(Location::test(2, 1)),
                "Data".to_owned(),
                vec![(
                    "a".to_owned(),
                    Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
                )],
                None,
                Scope::new("Data".to_owned(), ScopeType::Structure, None).wrap(),
            )
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_operator_field_1st_operand_expected_structure() {
    let input = r#"
fn main() {
    let tuple = (true, true, false);
    let value = tuple.first;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::OperatorDotFirstOperandExpectedInstance {
            location: Location::test(4, 17),
            found: Type::tuple(Some(Location::test(4, 17)), vec![Type::boolean(None); 3])
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_slice_start_out_of_range() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[-(1 as i8) .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArraySliceStartOutOfRange {
        location: Location::test(4, 25),
        start: BigInt::from(-1).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_slice_end_out_of_range() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[0 .. 6];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ArraySliceEndOutOfRange {
        location: Location::test(4, 23),
        end: BigInt::from(6).to_string(),
        size: 5,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_slice_end_lesser_than_start() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ArraySliceEndLesserThanStart {
            location: Location::test(4, 23),
            start: BigInt::from(2).to_string(),
            end: BigInt::from(1).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_tuple_field_out_of_range() {
    let input = r#"
fn main() {
    let tuple = (1, 2, 3);
    let result = tuple.5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TupleFieldOutOfRange {
        location: Location::test(4, 24),
        r#type: Type::tuple(
            Some(Location::test(4, 24)),
            vec![Type::integer_unsigned(None, zinc_const::bitlength::BYTE); 3],
        )
        .to_string(),
        field_index: 5,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_structure_field_does_not_exist() {
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

    let expected = Err(Error::Semantic(SemanticError::StructureFieldDoesNotExist {
        location: Location::test(10, 22),
        r#type: "Data".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_field_does_not_exist() {
    let input = r#"
contract Test {
    a: u8;

    fn test(self) -> u8 {
        self.b
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureFieldDoesNotExist {
        location: Location::test(6, 14),
        r#type: "Test".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
