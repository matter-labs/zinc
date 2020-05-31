use ff::PrimeField;
use ff::PrimeFieldRepr;
use sha2::Digest;
use sha2::Sha256;

use crate::gadgets::scalar::Scalar;
use crate::Engine;

pub fn sha256<E: Engine>(preimage: &[u8]) -> Vec<u8> {
    let mut hash = Sha256::new();
    hash.input(preimage);
    hash.result().to_vec()
}

pub fn sha256_of_concat<E: Engine>(left: &[u8], right: &[u8]) -> Vec<u8> {
    sha256::<E>(&[left, right].concat())
}

pub fn leaf_value_hash<E: Engine>(leaf_value: Vec<Scalar<E>>) -> Vec<u8> {
    let mut result = vec![];

    for field in leaf_value.into_iter() {
        let mut field_vec = vec![];
        field
            .get_value()
            .unwrap()
            .into_repr()
            .write_le(&mut field_vec)
            .unwrap();
        field_vec.resize(256 / 8, 0);

        let mut field_vec_be = vec![];
        for i in field_vec.into_iter() {
            let mut current_byte: u8 = 0;
            for j in 0..8 {
                current_byte <<= 1;
                current_byte += (i >> j) & 1u8;
            }
            field_vec_be.push(current_byte);
        }

        result.append(&mut field_vec_be);
    }

    sha256::<E>(&result)
}
