//!
//! The interpreter tests.
//!

#![cfg(test)]

use crate::interpreter::Error as InterpreterError;
use crate::lexical::Location;
use crate::Error;

#[test]
fn algorithm_factorial() {
    let code = r#"
inputs {}

let mut fact: field = 1;

for i in 2..6 {
    fact = fact * i as field;
};

require(fact == 120 as field);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn algorithm_fibonacci() {
    let code = r#"
inputs {}

let mut value_1: field = 0;
let mut value_2: field = 1;
let mut fibo = value_1;

for i in 1..=6 {
    fibo = value_1 + value_2;
    value_1 = value_2;
    value_2 = fibo;
};

require(fibo == 13 as field);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn array_double() {
    let code = r#"
inputs {}

let mut array_double: [[uint8; 4]; 4] = [
    [1, 2, 3, 4],
    [5, 6, 7, 8],
    [9, 10, 11, 12],
    [13, 14, 15, 16],
];

array_double[1][3] = 42;
array_double[2][2] = 111;
array_double[3][1] = 255;

require(array_double[1][3] == 42);
require(array_double[2][2] == 111);
require(array_double[3][1] == 255);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn array_filling() {
    let code = r#"
inputs {}

let original = [2, 1, 0, 3];
let mut array = [0; 4];

if 0 == original[0] {
    array[0] = 0;
};
if 1 == original[1] {
    array[1] = 1;
};
if 2 == original[2] {
    array[2] = 2;
};
if 3 == original[3] {
    array[3] = 3;
};

require(0 == array[0]);
require(1 == array[1]);
require(2 != array[2]);
require(3 == array[3]);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn block_expresssion_pyramid() {
    let code = r#"
inputs {}

let pyramid = 1 + {
    2 + {
        3 + {
            4
        } + 3
    } + 2
} + 1;

require(pyramid == 16);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn block_statement_mutating() {
    let code = r#"
inputs {}

let mut inner = 25;
{
    inner = 50;
};

require(inner == 50);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn conditional_expression_nested() {
    let code = r#"
inputs {}

let result = if false {
    1
} else if true {
    2
} else {
    3
};

require(result == 2);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn conditional_statement_elseless() {
    let code = r#"
inputs {}

let mut result = 5;
if false {
    result = 10;
};

require(result == 5);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn loop_inclusive() {
    let code = r#"
inputs {}

let mut sum: uint64 = 0;
for i in 0..=10 {
    sum = sum + i;
};

require(sum == 55 as uint64);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn loop_reverted() {
    let code = r#"
inputs {}

let mut sum: uint64 = 0;
for i in 10..0 {
    sum = sum + i;
};

require(sum == 55 as uint64);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn loop_with_while() {
    let code = r#"
inputs {}

let mut sum: uint64 = 0;
for i in 0..1_000_000 while i < 10 as uint64 {
    sum = sum + i;
};

require(sum == 55 as uint64);
"#;

    let expected = Ok(());

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}

#[test]
fn require_fail() {
    let code = r#"
inputs {}

let value = 42;

require(value != 42);
"#;

    let expected = Err(Error::Interpreter(InterpreterError::RequireFailed(
        Location::new(6, 1),
        "L6".to_owned(),
    )));

    let result = crate::interpret(crate::parse(code.to_owned()).expect("Syntax error"));

    assert_eq!(expected, result);
}
