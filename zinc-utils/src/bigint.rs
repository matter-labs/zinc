//!
//! The token conversion tools.
//!

use std::str::FromStr;

use num::bigint::ParseBigIntError;
use num::BigInt;
use num::Num;
use num::ToPrimitive;

///
/// Formats the amount with 18-digit precision, trimming the fractional zeros.
///
pub fn from_str(string: &str) -> Result<BigInt, ParseBigIntError> {
    let string = string.replace("_", "");

    if string.starts_with("0b") {
        BigInt::from_str_radix(&string["0b".len()..], zinc_const::base::BINARY)
    } else if string.starts_with("0o") {
        BigInt::from_str_radix(&string["0o".len()..], zinc_const::base::OCTAL)
    } else if string.starts_with("0x") {
        BigInt::from_str_radix(&string["0x".len()..], zinc_const::base::HEXADECIMAL)
    } else {
        let integer_and_rest: Vec<&str> = string.split('.').collect();
        let value = BigInt::from_str(integer_and_rest[0])?;

        let (value, exponent) = if integer_and_rest.len() > 1 {
            let fractional_and_exponent: Vec<&str> = integer_and_rest[1].split('E').collect();

            let exponent = if fractional_and_exponent.len() > 1 {
                let exponent = BigInt::from_str(fractional_and_exponent[1])?
                    .to_u32()
                    .expect("TODO: the exponent is too large");
                exponent as usize
            } else {
                0
            };

            let value = format!("{}{}", integer_and_rest[0], fractional_and_exponent[0]);
            let mut leading_zeros: usize = 0;
            for digit in value.chars() {
                if digit == '0' {
                    leading_zeros += 1;
                } else {
                    break;
                }
            }

            let exponent = if exponent > fractional_and_exponent[0].len() {
                exponent - fractional_and_exponent[0].len()
            } else {
                panic!("TODO: the exponent is too small");
            };

            let value = BigInt::from_str(
                value
                    .chars()
                    .skip(leading_zeros)
                    .collect::<String>()
                    .as_str(),
            )?;
            (value, exponent)
        } else {
            (value, 0)
        };

        Ok(value * BigInt::from(10).pow(exponent as u32))
    }
}
