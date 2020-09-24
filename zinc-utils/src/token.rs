//!
//! The token conversion tools.
//!

use std::str::FromStr;

use num::BigUint;

///
/// Converts the floating-point amount into a `BigUint` with `exponent`-digit precision.
///
pub fn parse_amount(amount: &str, exponent: u32) -> BigUint {
    let split = amount.split('.').collect::<Vec<&str>>();
    let exponent = exponent as usize;

    let string_wei_value = if split.len() == 1 {
        format!("{}{}", split[0], "0".repeat(exponent))
    } else if split.len() == 2 {
        assert!(
            split[1].len() <= exponent,
            "ETH amount can have up to 18 digits after the dot"
        );

        format!(
            "{}{}{}",
            split[0],
            split[1],
            "0".repeat(exponent - split[1].len())
        )
    } else {
        panic!("Too many dots in the ETH amount: {}", amount);
    };

    BigUint::from_str(&string_wei_value).expect("TODO: unreachable")
}
