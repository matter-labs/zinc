//!
//! The Schnorr signature tool library.
//!

#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

use rand::Rng;

use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::bellman::pairing::ff::PrimeFieldRepr;
use franklin_crypto::eddsa;
use franklin_crypto::jubjub;

///
/// Generates a random private key.
///
pub fn generate_private_key<E: jubjub::JubjubEngine>() -> eddsa::PrivateKey<E> {
    let mut rng = rand::OsRng::new().expect("failed to open randomness source");
    eddsa::PrivateKey(rng.gen())
}

///
/// Signs the `message` with the private `key`.
///
pub fn generate_signature<E: jubjub::JubjubEngine>(
    params: &E::Params,
    key: &eddsa::PrivateKey<E>,
    message: &[u8],
) -> eddsa::Signature<E> {
    let mut rng = rand::OsRng::new().expect("failed to open randomness source");

    let seed = eddsa::Seed::random_seed(&mut rng, message);
    let p_g = jubjub::FixedGenerators::SpendingKeyGenerator;

    key.sign_raw_message(
        message,
        &seed,
        p_g,
        params,
        E::Fs::CAPACITY as usize / zinc_const::bitlength::BYTE,
    )
}

///
/// Recovers the public key from the private key.
///
pub fn recover_public_key<E: jubjub::JubjubEngine>(
    params: &E::Params,
    private_key: &eddsa::PrivateKey<E>,
) -> eddsa::PublicKey<E> {
    let p_g = jubjub::FixedGenerators::SpendingKeyGenerator;
    eddsa::PublicKey::from_private(private_key, p_g, params)
}

///
/// Converts an `Fr` to a hexadecimal string.
///
pub fn fr_into_hex<Fr: PrimeField>(fr: Fr) -> String {
    let mut buffer = Vec::<u8>::new();

    fr.into_repr()
        .write_be(&mut buffer)
        .expect("failed to write into Vec<u8>");

    let num = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &buffer);
    format!(
        "0x{}",
        num.to_str_radix(zinc_const::base::HEXADECIMAL as u32)
    )
}
