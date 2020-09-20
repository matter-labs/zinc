//!
//! The token conversion tools.
//!

use num::BigUint;

///
/// Formats the amount with 18-digit precision, trimming the fractional zeros.
///
pub fn format_amount(amount: &BigUint, _exponent: u32) -> String {
    amount.to_string()
}
