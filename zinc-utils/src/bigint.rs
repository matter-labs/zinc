//!
//! The token conversion tools.
//!

use num::bigint::ParseBigIntError;
use num::BigInt;
use num::Num;

///
/// Formats the amount with 18-digit precision, trimming the fractional zeros.
///
pub fn from_str_radix(string: &str) -> Result<BigInt, ParseBigIntError> {
    if string.starts_with("0b") {
        BigInt::from_str_radix(&string["0b".len()..], zinc_const::base::BINARY)
    } else if string.starts_with("0o") {
        BigInt::from_str_radix(&string["0o".len()..], zinc_const::base::OCTAL)
    } else if string.starts_with("0x") {
        BigInt::from_str_radix(&string["0x".len()..], zinc_const::base::HEXADECIMAL)
    } else {
        BigInt::from_str_radix(string, zinc_const::base::DECIMAL)
    }
}
