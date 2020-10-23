use sha2::Digest;
use sha2::Sha256;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;

use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn sha256<E: IEngine>(preimage: &[u8]) -> Vec<u8> {
    Sha256::digest(preimage).to_vec()
}

pub fn leaf_value_hash<E: IEngine>(leaf_value: Vec<Scalar<E>>) -> Vec<u8> {
    let mut result = Vec::with_capacity(zinc_const::bitlength::SHA256_HASH * leaf_value.len());

    for field in leaf_value.into_iter() {
        let mut field_vec = Vec::with_capacity(zinc_const::bitlength::SHA256_HASH);
        if let Some(fr) = field.get_value() {
            let _ = fr.into_repr().write_le(&mut field_vec);
        }

        let field_vec_module_eight = field_vec.len() % zinc_const::bitlength::BYTE;
        if field_vec_module_eight != 0 {
            field_vec.resize(
                field_vec.len() + zinc_const::bitlength::BYTE - field_vec_module_eight,
                0,
            );
        }

        let mut field_vec_be = Vec::with_capacity(field_vec.len());
        for i in field_vec.into_iter() {
            let mut current_byte: u8 = 0;
            for j in 0..zinc_const::bitlength::BYTE {
                current_byte <<= 1;
                current_byte += (i >> j) & 1u8;
            }
            field_vec_be.push(current_byte);
        }

        result.extend(field_vec_be);
    }

    sha256::<E>(&result)
}
