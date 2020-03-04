//!
//! The semantic analyzer integer constant element error.
//!

use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Error {
    TypesMismatchEquals(String, String),
    TypesMismatchNotEquals(String, String),
    TypesMismatchGreaterEquals(String, String),
    TypesMismatchLesserEquals(String, String),
    TypesMismatchGreater(String, String),
    TypesMismatchLesser(String, String),
    TypesMismatchAddition(String, String),
    TypesMismatchSubtraction(String, String),
    TypesMismatchMultiplication(String, String),
    TypesMismatchDivision(String, String),
    TypesMismatchRemainder(String, String),

    OverflowAddition { value: BigInt, r#type: String },
    OverflowSubtraction { value: BigInt, r#type: String },
    OverflowMultiplication { value: BigInt, r#type: String },
    OverflowDivision { value: BigInt, r#type: String },
    OverflowRemainder { value: BigInt, r#type: String },
    OverflowCasting { value: BigInt, r#type: String },
    OverflowNegation { value: BigInt, r#type: String },

    ForbiddenFieldRemainder,
    ForbiddenFieldNegation,

    ZeroDivision,
    ZeroRemainder,

    IntegerTooLarge { value: BigInt, bitlength: usize },
    UnsignedNegative { value: BigInt, r#type: String },
}
