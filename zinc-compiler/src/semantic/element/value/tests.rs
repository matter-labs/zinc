//!
//! The value element tests.
//!

#![cfg(test)]

use std::convert::TryFrom;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::Value;
use crate::semantic::Error as SemanticError;

#[test]
fn error_element_value_or_1st_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer || boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_or_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean || integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorOrSecondOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_xor_1st_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer ^^ boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorXorFirstOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_xor_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean ^^ integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorXorSecondOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_and_1st_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer && boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAndFirstOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_and_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean && integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAndSecondOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 23),
        ElementError::Value(ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType(
            Type::array(Type::integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let integer = 42;
    let unit = ();
    let value = unit == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedUnit(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = boolean == integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = false;
    let value = integer == boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_not_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let array = [1, 2, 3];
    let integer = 42;
    let value = array != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 23),
        ElementError::Value(
            ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType(
                Type::array(Type::integer_unsigned(crate::BITLENGTH_BYTE), 3).to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_not_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let integer = 42;
    let unit = ();
    let value = unit != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_not_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = true;
    let value = boolean != integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_not_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let integer = 42;
    let boolean = false;
    let value = integer != boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_greater_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean >= integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorGreaterEqualsFirstOperandExpectedInteger(
                Type::boolean().to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_greater_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer >= boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorGreaterEqualsSecondOperandExpectedInteger(
                Type::boolean().to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_lesser_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean <= integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorLesserEqualsFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_lesser_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer <= boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorLesserEqualsSecondOperandExpectedInteger(
                Type::boolean().to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_greater_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean > integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorGreaterFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_greater_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer > boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorGreaterSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_lesser_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean < integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorLesserFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_lesser_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer < boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorLesserSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_addition_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean + integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAdditionFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_addition_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer + boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorAdditionSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_subtraction_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean - integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorSubtractionFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_subtraction_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer - boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorSubtractionSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_multiplication_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean * integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorMultiplicationFirstOperandExpectedInteger(
                Type::boolean().to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_multiplication_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer * boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(
            ValueError::OperatorMultiplicationSecondOperandExpectedInteger(
                Type::boolean().to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_division_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean / integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorDivisionFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_division_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer / boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorDivisionSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_remainder_1st_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = boolean % integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorRemainderFirstOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_remainder_2nd_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let integer = 42;
    let value = integer % boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 25),
        ElementError::Value(ValueError::OperatorRemainderSecondOperandExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_negation_expected_integer() {
    let input = r#"
fn main() {
    let boolean = true;
    let value = -boolean;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::OperatorNegationExpectedInteger(
            Type::boolean().to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_not_expected_boolean() {
    let input = r#"
fn main() {
    let integer = 42;
    let value = !integer;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 17),
        ElementError::Value(ValueError::OperatorNotExpectedBoolean(
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_index_1st_expected_array() {
    let input = r#"
fn main() {
    let value = (true, false, true)[1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorIndexFirstOperandExpectedArray(
            Value::try_from(&Type::tuple(vec![Type::boolean(); 3]))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_index_2nd_expected_integer_or_range() {
    let input = r#"
fn main() {
    let value = [1, 2, 3][true];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Value(
            ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange(
                Constant::Boolean(true).to_string(),
            ),
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_field_1st_expected_tuple() {
    let input = r#"
fn main() {
    let value = [true, true, false].1;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedTuple(
            Value::try_from(&Type::array(Type::boolean(), 3))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_value_field_1st_expected_structure() {
    let input = r#"
fn main() {
    let value = [true, true, false].first;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedStructure(
            Value::try_from(&Type::array(Type::boolean(), 3))
                .expect(crate::semantic::tests::PANIC_TEST_DATA)
                .to_string(),
        )),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}
