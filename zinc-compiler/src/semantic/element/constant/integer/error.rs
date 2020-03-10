//!
//! The semantic analyzer integer constant element error.
//!

use num_bigint::BigInt;

#[derive(Debug, PartialEq)]
pub enum Error {
    TypesMismatchEquals { first: String, second: String },
    TypesMismatchNotEquals { first: String, second: String },
    TypesMismatchGreaterEquals { first: String, second: String },
    TypesMismatchLesserEquals { first: String, second: String },
    TypesMismatchGreater { first: String, second: String },
    TypesMismatchLesser { first: String, second: String },
    TypesMismatchAddition { first: String, second: String },
    TypesMismatchSubtraction { first: String, second: String },
    TypesMismatchMultiplication { first: String, second: String },
    TypesMismatchDivision { first: String, second: String },
    TypesMismatchRemainder { first: String, second: String },

    OverflowAddition { value: BigInt, r#type: String },
    OverflowSubtraction { value: BigInt, r#type: String },
    OverflowMultiplication { value: BigInt, r#type: String },
    OverflowDivision { value: BigInt, r#type: String },
    OverflowRemainder { value: BigInt, r#type: String },
    OverflowCasting { value: BigInt, r#type: String },
    OverflowNegation { value: BigInt, r#type: String },

    ForbiddenFieldDivision,
    ForbiddenFieldRemainder,
    ForbiddenFieldNegation,

    ZeroDivision,
    ZeroRemainder,

    IntegerTooLarge { value: BigInt, bitlength: usize },
    UnsignedNegative { value: BigInt, r#type: String },
}
