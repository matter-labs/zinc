use std::ops::Div;
use std::ops::Neg;

use num::bigint::Sign;
use num::BigInt;
use num::Signed;

use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;
use franklin_crypto::bellman::pairing::Engine;

pub fn fr_to_bigint<E: Engine>(fr: &E::Fr, signed: bool) -> BigInt {
    if signed {
        fr_to_bigint_signed::<E>(fr)
    } else {
        fr_to_bigint_unsigned::<E>(fr)
    }
}

pub fn bigint_to_fr<E: Engine>(bigint: &BigInt) -> Option<E::Fr> {
    if bigint.is_positive() {
        E::Fr::from_str(&bigint.to_str_radix(10))
    } else {
        let abs = E::Fr::from_str(&bigint.neg().to_str_radix(10))?;
        let mut fr = E::Fr::zero();
        fr.sub_assign(&abs);
        Some(fr)
    }
}

fn fr_to_bigint_signed<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    E::Fr::char()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let modulus = BigInt::from_bytes_be(Sign::Plus, &buffer);
    buffer.clear();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    let value = BigInt::from_bytes_be(Sign::Plus, &buffer);

    if value < (modulus.clone().div(2)) {
        value
    } else {
        value - modulus
    }
}

fn fr_to_bigint_unsigned<E: Engine>(fr: &E::Fr) -> BigInt {
    let mut buffer = Vec::<u8>::new();
    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");
    BigInt::from_bytes_be(Sign::Plus, &buffer)
}
