//!
//! The `contract` statement tests.
//!

#[test]
fn ok_empty() {
    let input = r#"
contract Uniswap {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_single_field() {
    let input = r#"
contract Uniswap {
    a: u8;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple_fields() {
    let input = r#"
contract Uniswap {
    a: u8;
    b: u8;
    c: u8;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_single_constant() {
    let input = r#"
contract Uniswap {
    const A: u8 = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple_constants() {
    let input = r#"
contract Uniswap {
    const A: u8 = 42;
    const B: u8 = 42;
    const C: u8 = 42;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_single_function_private() {
    let input = r#"
contract Uniswap {
    fn f1() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple_functions_private() {
    let input = r#"
contract Uniswap {
    fn f1() -> u8 { 42 }

    fn f2() -> u8 { 42 }

    fn f3() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_single_function_public() {
    let input = r#"
contract Uniswap {
    pub fn f1() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple_functions_public() {
    let input = r#"
contract Uniswap {
    pub fn f1() -> u8 { 42 }

    pub fn f2() -> u8 { 42 }

    pub fn f3() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_single_each() {
    let input = r#"
contract Uniswap {
    a: u8;

    const A: u8 = 42;

    fn _f1() -> u8 { 42 }

    pub fn f1() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple_each() {
    let input = r#"
contract Uniswap {
    a: u8;
    b: u8;
    c: u8;

    const A: u8 = 42;
    const B: u8 = 42;
    const C: u8 = 42;

    fn _f1() -> u8 { 42 }
    fn _f2() -> u8 { 42 }
    fn _f3() -> u8 { 42 }

    pub fn f1() -> u8 { 42 }
    pub fn f2() -> u8 { 42 }
    pub fn f3() -> u8 { 42 }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}
