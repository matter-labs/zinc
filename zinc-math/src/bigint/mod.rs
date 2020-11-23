//!
//! The BigInt parsing tools.
//!

#[cfg(test)]
mod tests;

use std::str::FromStr;

use num::BigInt;
use num::Num;
use num::Zero;

use crate::error::Error;

///
/// The extended BigInt parsing function, which supports:
///
/// - binary, octal, hexadecimal numbers
/// - trimming out underscores
/// - decimal numbers, where the exponent is not less than the number of fractional digits
///
pub fn from_str(string: &str) -> crate::Result<BigInt> {
    let string = string.replace("_", "");

    if let Some(string) = string.strip_prefix("0b") {
        Ok(BigInt::from_str_radix(string, zinc_const::base::BINARY)?)
    } else if let Some(string) = string.strip_prefix("0o") {
        Ok(BigInt::from_str_radix(string, zinc_const::base::OCTAL)?)
    } else if let Some(string) = string.strip_prefix("0x") {
        Ok(BigInt::from_str_radix(
            string,
            zinc_const::base::HEXADECIMAL,
        )?)
    } else {
        let number_and_exponent: Vec<&str> = string.split('E').collect();
        let integer_and_fractional: Vec<&str> = number_and_exponent[0].split('.').collect();

        let (fractional, mut fractional_digits) = if integer_and_fractional.len() > 1 {
            (
                BigInt::from_str(integer_and_fractional[1])?,
                integer_and_fractional[1].len(),
            )
        } else {
            (BigInt::zero(), 0)
        };
        if fractional.is_zero() {
            fractional_digits = 0;
        }

        let exponent: u32 = if number_and_exponent.len() > 1 {
            number_and_exponent[1].parse()?
        } else {
            0
        };

        let mut value_str = integer_and_fractional[0].to_owned();
        if integer_and_fractional.len() > 1 && !fractional.is_zero() {
            value_str += integer_and_fractional[1];
        }

        let mut leading_zeros: usize = 0;
        for digit in value_str.chars() {
            if digit == '0' {
                leading_zeros += 1;
            } else {
                break;
            }
        }

        let exponent = if exponent >= fractional_digits as u32 {
            exponent - fractional_digits as u32
        } else {
            return Err(Error::ExponentTooSmall(exponent));
        };

        let value = if value_str.len() == leading_zeros {
            BigInt::zero()
        } else {
            BigInt::from_str(
                value_str
                    .chars()
                    .skip(leading_zeros)
                    .collect::<String>()
                    .as_str(),
            )?
        };

        Ok(value * BigInt::from(10).pow(exponent))
    }
}
