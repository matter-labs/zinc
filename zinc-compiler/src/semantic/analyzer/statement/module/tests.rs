//!
//! The `mod` statement tests.
//!

use std::collections::HashMap;
use std::path::PathBuf;

use crate::source::Source;

#[test]
fn ok_module_constants_flat() {
    let one = r#"
const VALUE: u8 = 25;
"#;

    let two = r#"
const VALUE: u8 = 42;
"#;

    let three = r#"
const VALUE: u8 = 64;
"#;

    let entry = r#"
mod one;
mod two;
mod three;

fn main() -> u8 {
    one::VALUE + two::VALUE + three::VALUE
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "one".to_owned(),
                Source::test(one, PathBuf::from("one.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "two".to_owned(),
                Source::test(two, PathBuf::from("two.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "three".to_owned(),
                Source::test(three, PathBuf::from("three.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_constants_vertical() {
    let level_1 = r#"
mod level_2;

const VALUE: u8 = 25;
"#;

    let level_2 = r#"
mod level_3;

const VALUE: u8 = 42;
"#;

    let level_3 = r#"
const VALUE: u8 = 64;
"#;

    let entry = r#"
mod level_1;

fn main() -> u8 {
    level_1::VALUE + level_1::level_2::VALUE + level_1::level_2::level_3::VALUE
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "level_1".to_owned(),
            Source::test(
                level_1,
                PathBuf::from("level_1.zn"),
                vec![(
                    "level_2".to_owned(),
                    Source::test(
                        level_2,
                        PathBuf::from("level_1/level_2.zn"),
                        vec![(
                            "level_3".to_owned(),
                            Source::test(
                                level_3,
                                PathBuf::from("level_1/level_2/level_3.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        )]
                        .into_iter()
                        .collect::<HashMap<String, Source>>()
                    )
                    .expect(zinc_const::panic::TEST_DATA_VALID)
                )]
                .into_iter()
                .collect::<HashMap<String, Source>>()
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_constants_tree() {
    let level_1_first = r#"
mod level_2_first;
mod level_2_second;

const VALUE: u8 = 1;
"#;

    let level_1_second = r#"
mod level_2_first;
mod level_2_second;

const VALUE: u8 = 2;
"#;
    let level_1_third = r#"
mod level_2_first;
mod level_2_second;

const VALUE: u8 = 3;
"#;

    let first_level_2_first = r#"
const VALUE: u8 = 4;
"#;
    let first_level_2_second = r#"
const VALUE: u8 = 5;
"#;

    let second_level_2_first = r#"
const VALUE: u8 = 7;
"#;
    let second_level_2_second = r#"
const VALUE: u8 = 8;
"#;

    let third_level_2_first = r#"
const VALUE: u8 = 10;
"#;
    let third_level_2_second = r#"
const VALUE: u8 = 11;
"#;

    let entry = r#"
mod level_1_first;
mod level_1_second;
mod level_1_third;

fn main() -> u8 {
    level_1_first::VALUE +
        level_1_first::level_2_first::VALUE +
        level_1_first::level_2_second::VALUE +
    level_1_second::VALUE +
        level_1_second::level_2_first::VALUE +
        level_1_second::level_2_second::VALUE +
    level_1_third::VALUE +
        level_1_third::level_2_first::VALUE +
        level_1_third::level_2_second::VALUE
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "level_1_first".to_owned(),
                Source::test(
                    level_1_first,
                    PathBuf::from("level_1_first.zn"),
                    vec![
                        (
                            "level_2_first".to_owned(),
                            Source::test(
                                first_level_2_first,
                                PathBuf::from("level_1_first/level_2_first.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                        (
                            "level_2_second".to_owned(),
                            Source::test(
                                first_level_2_second,
                                PathBuf::from("level_1_first/level_2_second.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "level_1_second".to_owned(),
                Source::test(
                    level_1_second,
                    PathBuf::from("level_1_second.zn"),
                    vec![
                        (
                            "level_2_first".to_owned(),
                            Source::test(
                                second_level_2_first,
                                PathBuf::from("level_1_second/level_2_first.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                        (
                            "level_2_second".to_owned(),
                            Source::test(
                                second_level_2_second,
                                PathBuf::from("level_1_second/level_2_second.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "level_1_third".to_owned(),
                Source::test(
                    level_1_third,
                    PathBuf::from("level_1_third.zn"),
                    vec![
                        (
                            "level_2_first".to_owned(),
                            Source::test(
                                third_level_2_first,
                                PathBuf::from("level_1_third/level_2_first.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                        (
                            "level_2_second".to_owned(),
                            Source::test(
                                third_level_2_second,
                                PathBuf::from("level_1_third/level_2_second.zn"),
                                HashMap::new()
                            )
                            .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            )
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_structures_flat() {
    let one = r#"
struct One {
    value: u8,
}
"#;

    let two = r#"
struct Two {
    value: u8,
}
"#;

    let three = r#"
struct Three {
    value: u8,
}
"#;

    let entry = r#"
mod one;
mod two;
mod three;

struct Together {
    a: self::one::One,
    b: self::two::Two,
    c: self::three::Three,
}

fn main() -> Together {
    Together {
        a: self::one::One {
            value: 10,
        },
        b: self::two::Two {
            value: 20,
        },
        c: self::three::Three {
            value: 30,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "one".to_owned(),
                Source::test(one, PathBuf::from("one.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "two".to_owned(),
                Source::test(two, PathBuf::from("two.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "three".to_owned(),
                Source::test(three, PathBuf::from("three.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_structures_flat_imported() {
    let one = r#"
struct One {
    value: u8,
}
"#;

    let two = r#"
struct Two {
    value: u8,
}
"#;

    let three = r#"
struct Three {
    value: u8,
}
"#;

    let entry = r#"
mod one;
mod two;
mod three;

use self::one::One;
use self::two::Two;
use self::three::Three;

struct Together {
    a: One,
    b: Two,
    c: Three,
}

fn main() -> Together {
    Together {
        a: One {
            value: 10,
        },
        b: Two {
            value: 20,
        },
        c: Three {
            value: 30,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "one".to_owned(),
                Source::test(one, PathBuf::from("one.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "two".to_owned(),
                Source::test(two, PathBuf::from("two.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "three".to_owned(),
                Source::test(three, PathBuf::from("three.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_structures_vertical() {
    let one = r#"
mod two;

struct One {
    b: self::two::Two,
}
"#;

    let two = r#"
mod three;

struct Two {
    c: self::three::Three,
}
"#;

    let three = r#"
struct Three {
    value: u8,
}
"#;

    let entry = r#"
mod one;

struct Together {
    a: self::one::One,
}

fn main() -> Together {
    Together {
        a: one::One {
            b: one::two::Two {
                c: one::two::three::Three {
                    value: 30,
                },
            },
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "one".to_owned(),
            Source::test(
                one,
                PathBuf::from("one.zn"),
                vec![(
                    "two".to_owned(),
                    Source::test(
                        two,
                        PathBuf::from("one/two.zn"),
                        vec![(
                            "three".to_owned(),
                            Source::test(three, PathBuf::from("one/two/three.zn"), HashMap::new())
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                        ),]
                        .into_iter()
                        .collect::<HashMap<String, Source>>()
                    )
                    .expect(zinc_const::panic::TEST_DATA_VALID)
                ),]
                .into_iter()
                .collect::<HashMap<String, Source>>()
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_module_structures_vertical_imported() {
    let one = r#"
mod two;

use self::two::Two;

struct One {
    b: Two,
}
"#;

    let two = r#"
mod three;

use self::three::Three;

struct Two {
    c: Three,
}
"#;

    let three = r#"
struct Three {
    value: u8,
}
"#;

    let entry = r#"
mod one;

use self::one::One;
use self::one::two::Two;
use self::one::two::three::Three;

struct Together {
    a: One,
}

fn main() -> Together {
    Together {
        a: One {
            b: Two {
                c: Three {
                    value: 30,
                },
            },
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "one".to_owned(),
            Source::test(
                one,
                PathBuf::from("one.zn"),
                vec![(
                    "two".to_owned(),
                    Source::test(
                        two,
                        PathBuf::from("one/two.zn"),
                        vec![(
                            "three".to_owned(),
                            Source::test(three, PathBuf::from("one/two/three.zn"), HashMap::new())
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                        )]
                        .into_iter()
                        .collect::<HashMap<String, Source>>()
                    )
                    .expect(zinc_const::panic::TEST_DATA_VALID)
                ),]
                .into_iter()
                .collect::<HashMap<String, Source>>()
            )
            .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_root_scope() {
    let other = r#"
use crate::RootData;

struct Other {
    data: RootData,
}
"#;

    let entry = r#"
mod other;

use self::other::Other;

struct RootData {
    value: u8
}

fn main() -> Other {
    Other {
        data: RootData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_super_scope() {
    let other = r#"
use super::RootData;

struct Other {
    data: RootData,
}
"#;

    let entry = r#"
mod other;

use self::other::Other;

struct RootData {
    value: u8
}

fn main() -> Other {
    Other {
        data: RootData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_root_contract_function() {
    let other = r#"
use crate::RootData;

struct Other {
    data: RootData,
}

impl Other {
    fn access() -> u8 {
        crate::Test::default()
    }
}
"#;

    let entry = r#"
mod other;

use self::other::Other;

struct RootData {
    value: u8
}

contract Test {
    pub fn default() -> u8 {
        42
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_super_contract_function() {
    let other = r#"
use super::RootData;

struct Other {
    data: RootData,
}

impl Other {
    fn access() -> u8 {
        super::Test::default()
    }
}
"#;

    let entry = r#"
mod other;

use self::other::Other;

struct RootData {
    value: u8
}

contract Test {
    pub fn default() -> u8 {
        42
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "other".to_owned(),
            Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID)
        ),]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_root_scope() {
    let other = r#"
use crate::accessed::AccessedData;

struct Other {
    data: AccessedData,
}
"#;

    let accessed = r#"
struct AccessedData {
    value: u8
}
"#;

    let entry = r#"
mod other;
mod accessed;

use self::other::Other;
use self::accessed::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other".to_owned(),
                Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed".to_owned(),
                Source::test(accessed, PathBuf::from("accessed.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_super_scope() {
    let other = r#"
use super::accessed::AccessedData;

struct Other {
    data: AccessedData,
}
"#;

    let accessed = r#"
struct AccessedData {
    value: u8
}
"#;

    let entry = r#"
mod other;
mod accessed;

use self::other::Other;
use self::accessed::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other".to_owned(),
                Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed".to_owned(),
                Source::test(accessed, PathBuf::from("accessed.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_root_scope_three_levels() {
    let other_level_1 = r#"
mod other_level_2;
"#;

    let other_level_2 = r#"
mod other_level_3;
"#;

    let other_level_3 = r#"
use crate::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

struct Other {
    data: AccessedData,
}
"#;

    let accessed_level_1 = r#"
mod accessed_level_2;
"#;

    let accessed_level_2 = r#"
mod accessed_level_3;
"#;

    let accessed_level_3 = r#"
struct AccessedData {
    value: u8
}
"#;

    let entry = r#"
mod other_level_1;
mod accessed_level_1;

use self::other_level_1::other_level_2::other_level_3::Other;
use self::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other_level_1".to_owned(),
                Source::test(
                    other_level_1,
                    PathBuf::from("other_level_1.zn"),
                    vec![(
                        "other_level_2".to_owned(),
                        Source::test(
                            other_level_2,
                            PathBuf::from("other_level_1/other_level_2.zn"),
                            vec![(
                                "other_level_3".to_owned(),
                                Source::test(
                                    other_level_3,
                                    PathBuf::from("other_level_1/other_level_2/other_level_3.zn"),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed_level_1".to_owned(),
                Source::test(
                    accessed_level_1,
                    PathBuf::from("accessed_level_1.zn"),
                    vec![(
                        "accessed_level_2".to_owned(),
                        Source::test(
                            accessed_level_2,
                            PathBuf::from("accessed_level_1/accessed_level_2.zn"),
                            vec![(
                                "accessed_level_3".to_owned(),
                                Source::test(
                                    accessed_level_3,
                                    PathBuf::from(
                                        "accessed_level_1/accessed_level_2/accessed_level_3.zn"
                                    ),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_super_scope_three_levels() {
    let other_level_1 = r#"
mod other_level_2;
"#;

    let other_level_2 = r#"
mod other_level_3;
"#;

    let other_level_3 = r#"
use super::super::super::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

struct Other {
    data: AccessedData,
}
"#;

    let accessed_level_1 = r#"
mod accessed_level_2;
"#;

    let accessed_level_2 = r#"
mod accessed_level_3;
"#;

    let accessed_level_3 = r#"
struct AccessedData {
    value: u8
}
"#;

    let entry = r#"
mod other_level_1;
mod accessed_level_1;

use self::other_level_1::other_level_2::other_level_3::Other;
use self::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other_level_1".to_owned(),
                Source::test(
                    other_level_1,
                    PathBuf::from("other_level_1.zn"),
                    vec![(
                        "other_level_2".to_owned(),
                        Source::test(
                            other_level_2,
                            PathBuf::from("other_level_1/other_level_2.zn"),
                            vec![(
                                "other_level_3".to_owned(),
                                Source::test(
                                    other_level_3,
                                    PathBuf::from("other_level_1/other_level_2/other_level_3.zn"),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed_level_1".to_owned(),
                Source::test(
                    accessed_level_1,
                    PathBuf::from("accessed_level_1.zn"),
                    vec![(
                        "accessed_level_2".to_owned(),
                        Source::test(
                            accessed_level_2,
                            PathBuf::from("accessed_level_1/accessed_level_2.zn"),
                            vec![(
                                "accessed_level_3".to_owned(),
                                Source::test(
                                    accessed_level_3,
                                    PathBuf::from(
                                        "accessed_level_1/accessed_level_2/accessed_level_3.zn"
                                    ),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_root_scope_impl_function() {
    let other = r#"
use crate::accessed::AccessedData;

struct Other {
    data: AccessedData,
}

impl Other {
    pub fn access() -> u8 {
        AccessedData::default()
    }
}
"#;

    let accessed = r#"
struct AccessedData {
    value: u8
}

impl AccessedData {
    pub fn default() -> u8 {
        42
    }
}
"#;

    let entry = r#"
mod other;
mod accessed;

use self::other::Other;
use self::accessed::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other".to_owned(),
                Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed".to_owned(),
                Source::test(accessed, PathBuf::from("accessed.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_super_scope_impl_function() {
    let other = r#"
use super::accessed::AccessedData;

struct Other {
    data: AccessedData,
}

impl Other {
    pub fn access() -> u8 {
        AccessedData::default()
    }
}
"#;

    let accessed = r#"
struct AccessedData {
    value: u8
}

impl AccessedData {
    pub fn default() -> u8 {
        42
    }
}
"#;

    let entry = r#"
mod other;
mod accessed;

use self::other::Other;
use self::accessed::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other".to_owned(),
                Source::test(other, PathBuf::from("other.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed".to_owned(),
                Source::test(accessed, PathBuf::from("accessed.zn"), HashMap::new())
                    .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_root_scope_impl_function_three_levels_multiple_access() {
    let other_level_1 = r#"
mod other_level_2;
"#;

    let other_level_2 = r#"
mod other_level_3;
"#;

    let other_level_3 = r#"
use crate::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

struct Other {
    data: AccessedData,
}

impl Other {
    pub fn access() -> u8 {
        AccessedData::default()
    }
}
"#;

    let accessed_level_1 = r#"
mod accessed_level_2;
"#;

    let accessed_level_2 = r#"
mod accessed_level_3;
"#;

    let accessed_level_3 = r#"
struct AccessedData {
    value: u8
}

impl AccessedData {
    pub fn default() -> u8 {
        42
    }
}
"#;

    let entry = r#"
mod other_level_1;
mod accessed_level_1;

use self::other_level_1::other_level_2::other_level_3::Other;
use self::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other_level_1".to_owned(),
                Source::test(
                    other_level_1,
                    PathBuf::from("other_level_1.zn"),
                    vec![(
                        "other_level_2".to_owned(),
                        Source::test(
                            other_level_2,
                            PathBuf::from("other_level_1/other_level_2.zn"),
                            vec![(
                                "other_level_3".to_owned(),
                                Source::test(
                                    other_level_3,
                                    PathBuf::from("other_level_1/other_level_2/other_level_3.zn"),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed_level_1".to_owned(),
                Source::test(
                    accessed_level_1,
                    PathBuf::from("accessed_level_1.zn"),
                    vec![(
                        "accessed_level_2".to_owned(),
                        Source::test(
                            accessed_level_2,
                            PathBuf::from("accessed_level_1/accessed_level_2.zn"),
                            vec![(
                                "accessed_level_3".to_owned(),
                                Source::test(
                                    accessed_level_3,
                                    PathBuf::from(
                                        "accessed_level_1/accessed_level_2/accessed_level_3.zn"
                                    ),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}

#[test]
fn ok_access_through_super_scope_impl_function_three_levels_multiple_access() {
    let other_level_1 = r#"
mod other_level_2;
"#;

    let other_level_2 = r#"
mod other_level_3;
"#;

    let other_level_3 = r#"
use super::super::super::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

struct Other {
    data: AccessedData,
}

impl Other {
    pub fn access() -> u8 {
        AccessedData::default()
    }
}
"#;

    let accessed_level_1 = r#"
mod accessed_level_2;
"#;

    let accessed_level_2 = r#"
mod accessed_level_3;
"#;

    let accessed_level_3 = r#"
struct AccessedData {
    value: u8
}

impl AccessedData {
    pub fn default() -> u8 {
        42
    }
}
"#;

    let entry = r#"
mod other_level_1;
mod accessed_level_1;

use self::other_level_1::other_level_2::other_level_3::Other;
use self::accessed_level_1::accessed_level_2::accessed_level_3::AccessedData;

fn main() -> Other {
    Other {
        data: AccessedData {
            value: 42,
        },
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![
            (
                "other_level_1".to_owned(),
                Source::test(
                    other_level_1,
                    PathBuf::from("other_level_1.zn"),
                    vec![(
                        "other_level_2".to_owned(),
                        Source::test(
                            other_level_2,
                            PathBuf::from("other_level_1/other_level_2.zn"),
                            vec![(
                                "other_level_3".to_owned(),
                                Source::test(
                                    other_level_3,
                                    PathBuf::from("other_level_1/other_level_2/other_level_3.zn"),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
            (
                "accessed_level_1".to_owned(),
                Source::test(
                    accessed_level_1,
                    PathBuf::from("accessed_level_1.zn"),
                    vec![(
                        "accessed_level_2".to_owned(),
                        Source::test(
                            accessed_level_2,
                            PathBuf::from("accessed_level_1/accessed_level_2.zn"),
                            vec![(
                                "accessed_level_3".to_owned(),
                                Source::test(
                                    accessed_level_3,
                                    PathBuf::from(
                                        "accessed_level_1/accessed_level_2/accessed_level_3.zn"
                                    ),
                                    HashMap::new()
                                )
                                .expect(zinc_const::panic::TEST_DATA_VALID)
                            ),]
                            .into_iter()
                            .collect::<HashMap<String, Source>>()
                        )
                        .expect(zinc_const::panic::TEST_DATA_VALID)
                    ),]
                    .into_iter()
                    .collect::<HashMap<String, Source>>()
                )
                .expect(zinc_const::panic::TEST_DATA_VALID)
            ),
        ]
        .into_iter()
        .collect::<HashMap<String, Source>>()
    )
    .is_ok());
}
