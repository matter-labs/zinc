//!
//! The scope tests.
//!

use std::collections::HashMap;
use std::path::PathBuf;

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;
use crate::source::Source;

#[test]
fn ok_current_scope() {
    let input = r#"
fn main() {
    const VALUE: u8 = 42;

    let result = VALUE;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_upper_scope() {
    let input = r#"
const VALUE: u8 = 42;

fn main() {
    let result = VALUE;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_far_scope() {
    let input = r#"
const VALUE: u8 = 42;

fn main() {
    {
        {
            {
                {
                    let result = VALUE;
                }
            }
        }
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_resolution_chain_constant() {
    let input = r#"
const A: u8 = B;
const B: u8 = C;
const C: u8 = 42;

fn main() {
    let result = C;
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_resolution_chain_type() {
    let input = r#"
struct Outer {
    a: u8,
    inner: Inner,
}

struct Inner {
    b: u8,
    inner: InnerMost
}

type InnerMost = field;

fn main() {
    let result = Outer {
        a: 42,
        inner: Inner {
            b: 25,
            inner: 0 as field,
        },
    };
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_resolution_chain_function() {
    let input = r#"
fn fourth() -> u8 { 42 }

fn second() -> u8 { third() }

fn first() -> u8 { second() }

fn third() -> u8 { fourth() }

fn main() -> u8 { first() }
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_constant_same_name_structure() {
    let input = r#"
struct Data {
    a: u8,
}

impl Data {
    const A: u8 = 42;

    pub fn default(self) {
        let A = 42;
    }
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_method_same_name_structure() {
    let input = r#"
struct Data {
    a: u8,
}

impl Data {
    pub fn default(self) {
        let next = 42;
    }

    pub fn next() {}
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_constant_same_name_enumeration() {
    let input = r#"
enum Data {
    A = 1,
}

impl Data {
    const B: u8 = 42;

    pub fn default(self) {
        let B = 42;
    }
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_variant_same_name_enumeration() {
    let input = r#"
enum Data {
    A = 1,
}

impl Data {
    pub fn default(self) {
        let A = 42;
    }
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_method_same_name_enumeration() {
    let input = r#"
enum Data {
    A = 1,
}

impl Data {
    pub fn default(self) {
        let next = 42;
    }

    pub fn next() {}
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_method_same_name_contract() {
    let input = r#"
contract Test {
    pub fn default(self) {
        let next = 42;
    }

    pub fn next() {}
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_constant_same_name_contract() {
    let input = r#"
contract Test {
    const A: u8 = 42;

    pub fn default(self) {
        let A = 42;
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_variable_field_same_name_contract() {
    let input = r#"
contract Test {
    a: u8;

    pub fn default(self) {
        let a = 42;
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_item_redeclared() {
    let input = r#"
fn main() {
    let result = 42;
    let result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemRedeclared {
        location: Location::test(4, 9),
        name: "result".to_owned(),
        reference: Some(Location::test(3, 9)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_redeclared_use() {
    let input = r#"
type X = u8;

use X;

fn main() -> X {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemRedeclared {
        location: Location::test(4, 5),
        name: "X".to_owned(),
        reference: Some(Location::test(2, 1)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_redeclared_use_with_alias() {
    let input = r#"
type X = u8;
type Y = u8;

use X as Y;

fn main() -> Y {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemRedeclared {
        location: Location::test(5, 10),
        name: "Y".to_owned(),
        reference: Some(Location::test(3, 1)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared() {
    let input = r#"
fn main() {
    result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(3, 5),
        name: "result".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_lower() {
    let input = r#"
fn main() {
    {
        let result = 42;
    }

    result = 64;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(7, 5),
        name: "result".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_enum_variant() {
    let input = r#"
enum Jabberwocky {
    Gone = 42,
}

fn main() {
    let really = Jabberwocky::Exists;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(7, 31),
        name: "Exists".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_enum_variant_outside() {
    let input = r#"
const Gone: u8 = 42;

enum Jabberwocky {}

fn main() {
    let really = Jabberwocky::Gone;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(7, 31),
        name: "Gone".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_self_lowercase() {
    let input = r#"
fn not_method(self) -> bool {
    42
}

fn main() {
    let value = not_method();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(2, 15),
        name: Keyword::SelfUppercase.to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_self_uppercase() {
    let input = r#"
fn not_method(value: Self) -> bool {
    42
}

fn main() {
    let value = not_method();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(2, 22),
        name: Keyword::SelfUppercase.to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_without_self() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

impl Data {
    pub fn sum(self) -> u8 {
        a + b
    }
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(9, 9),
        name: "a".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_constant() {
    let input = r#"
enum Data {
    A = 1,
    B = 2,
}

impl Data {
    const C: u8 = 3;

    pub fn sum() -> Self {
        Self::A + Self::B + C
    }
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(11, 29),
        name: "C".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_variant() {
    let input = r#"
enum Data {
    A = 1,
    B = 2,
}

impl Data {
    pub fn sum() -> Self {
        Self::A + B
    }
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(9, 19),
        name: "B".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_method() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

impl Data {
    pub fn get_a(self) -> u8 { self.a }

    pub fn get_b(self) -> u8 { self.b }

    pub fn sum(self) -> u8 {
        self.get_a() + get_b()
    }
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(13, 24),
        name: "get_b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_contract_constant() {
    let input = r#"
contract Test {
    const A: u8 = 42;

    pub fn default(self) -> u8 {
        A
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(6, 9),
        name: "A".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_contract_method() {
    let input = r#"
contract Test {
    a: u8;

    pub fn default(self) -> u8 {
        get_a()
    }

    fn get_a(self) -> u8 {
        self.a
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(6, 9),
        name: "get_a".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_undeclared_missing_self_contract_field() {
    let input = r#"
contract Test {
    a: u8;

    pub fn default(self) -> u8 {
        a
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemUndeclared {
        location: Location::test(6, 9),
        name: "a".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_item_is_not_a_namespace() {
    let input = r#"
const NOT_NAMESPACE: u8 = 42;

fn main() {
    let result = NOT_NAMESPACE::UNDEFINED;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeExpectedNamespace {
        location: Location::test(5, 18),
        name: "NOT_NAMESPACE".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_redeclared() {
    let input = r#"
contract Uniswap {
    pub fn deposit(amount: u248) -> bool { true }
}

contract Multiswap {
    pub fn deposit(amount: u248) -> bool { true }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeContractRedeclared {
        location: Location::test(6, 1),
        reference: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_constant_direct() {
    let input = r#"
const A: u8 = B;
const B: u8 = A;

fn main() -> u8 { B }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 7),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_constant_indirect() {
    let input = r#"
const A: u8 = B;
const B: u8 = C;
const C: u8 = D;
const D: u8 = A;

fn main() -> u8 { D }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 7),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_type_direct() {
    let input = r#"
type A = B;
type B = A;

fn main() -> A {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_type_indirect() {
    let input = r#"
struct Outer {
    a: u8,
    inner: Inner,
}

struct Inner {
    b: u8,
    inner: InnerMost
}

type InnerMost = Outer;

fn main() -> bool {
    false
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_mixed_direct() {
    let input = r#"
type Array = [u8; SIZE];

const SIZE: Array = [1, 2, 3, 4];

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_mixed_indirect() {
    let input = r#"
type Array = [u8; SIZE.value];

struct Size {
    value: u8,
    looped: Array,
}

const SIZE: Size = Size { value: 4 };

fn main() -> Array {
    [1, 2, 3, 4]
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_function_direct() {
    let input = r#"
fn first() -> u8 { second() }

fn second() -> u8 { first() }

fn main() -> u8 { first() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_function_indirect() {
    let input = r#"
fn fourth() -> u8 { first() }

fn second() -> u8 { third() }

fn first() -> u8 { second() }

fn third() -> u8 { fourth() }

fn main() -> u8 { first() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_implementation_function_direct() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    fn method(self, value: u8) -> u8 {
        self.another(value)
    }

    fn another(self, value: u8) -> u8 {
        self.method(value)
    }
}

fn main() -> Data {
    Data { value: Data { value: 42 }.method(54) }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(7, 5),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_implementation_function_indirect() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    fn method(self, value: u8) -> u8 {
        self.another(value)
    }

    fn another(self, value: u8) -> u8 {
        self.yet_another(value)
    }

    fn and_another(self, value: u8) -> u8 {
        self.method(value)
    }

    fn yet_another(self, value: u8) -> u8 {
        self.and_another(value)
    }
}

fn main() -> Data {
    Data { value: Data { value: 42 }.method(54) }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(7, 5),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_contract_function_direct() {
    let input = r#"
contract Data {
    value: u8;

    pub fn method(self) -> u8 {
        self.another(self.value)
    }

    pub fn another(self) -> u8 {
        self.method(self.value)
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(5, 5),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_contract_function_indirect() {
    let input = r#"
contract Data {
    value: u8;

    pub fn method(self) -> u8 {
        self.another(self.value)
    }

    pub fn another(self) -> u8 {
        self.yet_another(self.value)
    }

    pub fn and_another(self) -> u8 {
        self.method(self.value)
    }

    pub fn yet_another(self) -> u8 {
        self.and_another(self.value)
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(5, 5),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_function_between_modules_direct() {
    let other = r#"
fn call() -> u8 { crate::call() }
"#;

    let entry = r#"
mod other;

fn call() -> u8 { other::call() }

fn main() -> u8 { call() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID),
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>(),
    );

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_function_between_modules_indirect() {
    let third = r#"
fn call() -> u8 { crate::call() }
"#;

    let second = r#"
mod third;

fn call() -> u8 { third::call() }
"#;

    let first = r#"
mod second;

fn call() -> u8 { second::call() }
"#;

    let entry = r#"
mod first;

fn call() -> u8 { first::call() }

fn main() -> u8 { call() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(2, 1),
    }));

    let result = crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "first".to_owned(),
            Source::test(
                first,
                PathBuf::from("first.zn"),
                vec![(
                    "second".to_owned(),
                    Source::test(
                        second,
                        PathBuf::from("first/second.zn"),
                        vec![(
                            "third".to_owned(),
                            Source::test(
                                third,
                                PathBuf::from("first/second/third.zn"),
                                HashMap::new(),
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID),
                        )]
                        .into_iter()
                        .collect::<HashMap<String, Source>>(),
                    )
                    .expect(zinc_const::panic::TEST_DATA_VALID),
                )]
                .into_iter()
                .collect::<HashMap<String, Source>>(),
            )
            .expect(zinc_const::panic::TEST_DATA_VALID),
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>(),
    );

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_implementation_function_between_modules_direct() {
    let other = r#"
struct Other { value: u8 }

impl Other {
    pub fn call() -> u8 { crate::Call::call() }
}
"#;

    let entry = r#"
mod other;

use self::other::Other;

struct Call { value: u8 }

impl Call {
    pub fn call() -> u8 { self::other::Other::call() }
}

fn main() -> u8 { Call { value: 42 }.call() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(5, 5),
    }));

    let result = crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID),
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>(),
    );

    assert_eq!(result, expected);
}

#[test]
fn error_reference_loop_implementation_function_between_modules_indirect() {
    let third = r#"
use crate::Call;

struct Third { value: u8 }

impl Third {
    pub fn call() -> u8 { Call::call() }
}
"#;

    let second = r#"
mod third;

struct Second { value: u8 }

impl Second {
    pub fn call() -> u8 { third::Third::call() }
}
"#;

    let first = r#"
mod second;

struct First { value: u8 }

impl First {
    pub fn call() -> u8 { second::Second::call() }
}
"#;

    let entry = r#"
mod first;

struct Call { value: u8 }

impl Call {
    pub fn call() -> u8 { self::first::First::Call() }
}

fn main() -> u8 { Call { value: 42 }.call() }
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeReferenceLoop {
        location: Location::test(7, 5),
    }));

    let result = crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "first".to_owned(),
            Source::test(
                first,
                PathBuf::from("first.zn"),
                vec![(
                    "second".to_owned(),
                    Source::test(
                        second,
                        PathBuf::from("first/second.zn"),
                        vec![(
                            "third".to_owned(),
                            Source::test(
                                third,
                                PathBuf::from("first/second/third.zn"),
                                HashMap::new(),
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID),
                        )]
                        .into_iter()
                        .collect::<HashMap<String, Source>>(),
                    )
                    .expect(zinc_const::panic::TEST_DATA_VALID),
                )]
                .into_iter()
                .collect::<HashMap<String, Source>>(),
            )
            .expect(zinc_const::panic::TEST_DATA_VALID),
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>(),
    );

    assert_eq!(result, expected);
}
