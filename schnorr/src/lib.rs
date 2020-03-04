use franklin_crypto::bellman::pairing::ff::PrimeField;
use franklin_crypto::{eddsa, jubjub};
use rand::Rng;

pub fn generate_private_key<E: jubjub::JubjubEngine>() -> eddsa::PrivateKey<E> {
    let mut rng = rand::OsRng::new().expect("failed to open randomness source");
    eddsa::PrivateKey(rng.gen())
}

pub fn generate_signature<E: jubjub::JubjubEngine>(
    params: &E::Params,
    key: &eddsa::PrivateKey<E>,
    message: &[u8],
) -> eddsa::Signature<E> {
    let mut rng = rand::OsRng::new().expect("failed to open randomness source");

    let seed = eddsa::Seed::random_seed(&mut rng, message);
    let p_g = jubjub::FixedGenerators::SpendingKeyGenerator;

    key.sign_raw_message(message, &seed, p_g, params, E::Fs::CAPACITY as usize / 8)
}

pub fn recover_public_key<E: jubjub::JubjubEngine>(
    params: &E::Params,
    private_key: &eddsa::PrivateKey<E>,
) -> eddsa::PublicKey<E> {
    let p_g = jubjub::FixedGenerators::SpendingKeyGenerator;
    eddsa::PublicKey::from_private(private_key, p_g, params)
}
