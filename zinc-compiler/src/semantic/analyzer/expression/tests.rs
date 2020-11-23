//!
//! The expression tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Item as ScopeItem;

#[test]
fn ok_constant_element_simple() {
    let input = r#"
fn main() {
    const CONSTANT: u8 = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_constant_element_complex() {
    let input = r#"
fn main() {
    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = INNER_1 * INNER_2 + INNER_3 / INNER_4;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_constant_element_block() {
    let input = r#"
fn main() {
    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = {
        INNER_1 * INNER_2 + INNER_3 / INNER_4
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_constant_element_conditional() {
    let input = r#"
fn main() {
    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = if true {
        INNER_1 * INNER_2 + INNER_3 / INNER_4
    } else {
        64
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_constant_element_match() {
    let input = r#"
fn main() {
    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = match 42 {
        42 => INNER_1 * INNER_2 + INNER_3 / INNER_4,
        _ => 64,
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_constant_element_enum_variant() {
    let input = r#"
enum List {
    One = 1,
    Two = 2,
}

fn main() {
    const VALUE: List = List::Two;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_non_constant_element_simple() {
    let input = r#"
fn main() {
    let variable = 42;

    const CONSTANT: u8 = variable;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(5, 26),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_complex() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = INNER_1 * INNER_2 + variable - INNER_3 / INNER_4;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(10, 46),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_block() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = {
        INNER_1 * INNER_2 + variable - INNER_3 / INNER_4
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(11, 29),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_conditional_condition() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = if variable == 42 {
        INNER_1 * INNER_2 + INNER_3 / INNER_4
    } else {
        64
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(10, 29),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_conditional_main_branch() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = if true {
        INNER_1 * INNER_2 + variable - INNER_3 / INNER_4
    } else {
        64
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(11, 29),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_conditional_else_branch() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = if false {
        64
    } else {
        INNER_1 * INNER_2 + variable - INNER_3 / INNER_4
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(13, 29),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_match_scrutinee() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = match variable {
        42 => INNER_1 * INNER_2 + INNER_3 / INNER_4,
        _ => 64,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(10, 32),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_match_branch_ordinar() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = match 42 {
        42 => INNER_1 * INNER_2 + variable + INNER_3 / INNER_4,
        _ => 64,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(11, 35),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_constant_element_match_branch_wildcard() {
    let input = r#"
fn main() {
    let variable = 42;

    const INNER_1: u8 = 5;
    const INNER_2: u8 = 3;
    const INNER_3: u8 = 42;
    const INNER_4: u8 = 2;

    const CONSTANT: u8 = match 42 {
        42 => 42,
        _ => INNER_1 * INNER_2 + variable + INNER_3 / INNER_4,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(12, 34),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(3, 9)),
                false,
                "variable".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_storage_field_without_instance() {
    let input = r#"
contract Test {
    pub x: u8;

    pub fn new() -> Self {
        Self {
            x: 42,
        }
    }

    pub fn access() {
        let bug = Self::x;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ContractStorageFieldWithoutInstance {
            location: Location::test(12, 19),
            found: "x".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
