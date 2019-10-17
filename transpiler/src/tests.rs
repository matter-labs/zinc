//!
//! The transpiler tests.
//!

#![cfg(test)]

use crate::Transpiler;

#[test]
fn algorithm_factorial() {
    let input = r#"
inputs {}

let mut fact: field = 1;

for i in 2..6 {
    fact = fact * i as field;
};

require(fact == 120 as field);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn algorithm_fibonacci() {
    let input = r#"
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

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn array_double() {
    let input = r#"
inputs {}

let mut array_double: [[u8; 4]; 4] = [
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

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn array_filling() {
    let input = r#"
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
require(2 == array[2]);
require(3 == array[3]);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn tuple_nested() {
    let input = r#"
inputs {}

let mut tuple_nested: ((u8, u8), (u8, u8)) = (
    (1, 2),
    (3, 4),
);

tuple_nested.0.0 = 42;
tuple_nested.1.1 = 111;

require(tuple_nested.0.0 == 42);
require(tuple_nested.0.1 == 2);
require(tuple_nested.1.0 == 3);
require(tuple_nested.1.1 == 111);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn structure_mutating() {
    let input = r#"
inputs {}

struct Test {
    x: u8,
    y: u8,
    z: u8,
};

let mut test = Test {
    x: 1,
    y: 2,
    z: 3,
};

test.x = 5;
test.y = 7;
test.z = 9;
require(test.x == 5);
require(test.y == 7);
require(test.z == 9);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn block_expresssion_pyramid() {
    let input = r#"
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

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn block_statement_mutating() {
    let input = r#"
inputs {}

let mut inner = 25;
{
    inner = 50;
};

require(inner == 50);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn conditional_expression_nested() {
    let input = r#"
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

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn conditional_statement_elseless() {
    let input = r#"
inputs {}

let mut result = 5;
if false {
    result = 10;
};

require(result == 5);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn loop_inclusive() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;
for i in 0..=10 {
    sum = sum + i;
};

require(sum == 55 as u64);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn loop_reverted() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;
for i in 10..=0 {
    sum = sum + i;
};

require(sum == 55 as u64);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn loop_with_while() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;
for i in 0..1_000_000 while i < 10 as u64 {
    sum = sum + i;
};

require(sum == 55 as u64);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}

#[test]
fn loop_nested() {
    let input = r#"
inputs {}

let mut sum: u64 = 0;

for i in 0..=5 {
    sum = sum + i;
    for j in 0..=5 {
        sum = sum + j;
    };
};

require(sum == 105 as u64);
"#;

    let result =
        Transpiler::default().transpile(parser::parse(input.to_owned()).expect("Syntax error"));

    assert!(result.is_ok());
}
